use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::{ErrorBadRequest, ErrorForbidden};
use actix_web::{web, Error};
use futures::future::{ok, Either, Future, FutureResult};
use futures::Poll;
use std::cell::RefCell;
use std::rc::Rc;

use crate::db;
use crate::AppState;

pub struct CheckAuth;

impl<S, B> Transform<S> for CheckAuth
where
    //S: Service<Request = ServiceRequest, Response = ServiceResponse<B>>,
    S: Service<
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
    >,
    S: 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type InitError = ();
    type Transform = CheckAuthMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckAuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}
pub struct CheckAuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for CheckAuthMiddleware<S>
where
    //S: Service<Request = ServiceRequest, Response = ServiceResponse<B>>,
    S: Service<
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
    >,
    S::Future: 'static,
    S: 'static,
    Error: From<S::Error>,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut service = self.service.clone();
        let state: web::Data<AppState> = req.app_data().unwrap();
        if let Some(auth_token) = req.headers().get("token") {
            Box::new(
                state
                    .db
                    .send(db::token::FetchToken(
                        auth_token.to_str().unwrap().to_string(),
                    ))
                    .map_err(Error::from)
                    .and_then(move |token_res| match token_res {
                        Ok(_token) => {
                            //TODO: Validate token expiry time
                            Either::A(service.call(req).map_err(Error::from))
                        }
                        Err(_) => Either::B(ok(req.error_response(
                            ErrorForbidden("Please authenticate"),
                        ))),
                    }),
            )
        } else if req.path() == "/login" {
            //You can access the login even when not being authenticated
            Box::new(service.call(req).map_err(Error::from))
        } else {
            Box::new(ok(req.error_response(ErrorBadRequest(
                "Required headers are missing.",
            ))))
        }
    }
}

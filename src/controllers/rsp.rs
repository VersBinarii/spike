use actix_web::{web, Error as ActixErr, HttpResponse};
use futures::Future;

use std::error::Error;

use crate::db;
use crate::models::NewRsp;
use crate::AppState;

pub fn show_rsp(
    rsp_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::rsp::FetchRsp(rsp_id.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(rsp) => Ok(HttpResponse::Ok().json(rsp)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn list_rsp(
    q: web::Query<db::pagination::PaginateQuery>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::rsp::FetchRsps {
            page: q.page.unwrap_or(1),
            per_page: q.per_page.unwrap_or(10),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(rsps) => Ok(HttpResponse::Ok().json(rsps)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn create_new_rsp(
    req: web::Json<NewRsp>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::rsp::InsertRsp(req.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(new_rsp) => Ok(HttpResponse::Ok().json(new_rsp)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn update_rsp(
    (rsp_id, rsp_form): (web::Path<i32>, web::Json<NewRsp>),
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::rsp::UpdateRsp {
            rsp_id: rsp_id.into_inner(),
            rsp: rsp_form.into_inner(),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(updated_rsp) => Ok(HttpResponse::Ok().json(updated_rsp)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized},
    web, Error as ActixErr, HttpResponse,
};
use futures::{Future, IntoFuture};

use crate::db;
use crate::models::{LoginUser, LogoutUser};
use crate::AppState;

pub fn login(
    user: web::Json<LoginUser>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::user::FetchUser {
            username: user.username.to_owned(),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(fetched_user) => {
                if fetched_user.password == user.password {
                    Ok(fetched_user)
                } else {
                    return Err(ErrorUnauthorized(
                        "Failed to authenticate user.",
                    ));
                }
            }
            Err(_) => {
                return Err(ErrorInternalServerError(
                    "Failed to authenticate user.",
                ));
            }
        })
        .and_then(move |user| {
            state
                .db
                .send(db::token::InsertToken(user.username))
                .from_err()
                .and_then(|token_response| match token_response {
                    Ok(token) => Ok(HttpResponse::Ok().json(token)),
                    Err(_) => Err(ErrorInternalServerError(
                        "Failed to create session token",
                    )),
                })
        })
}

pub fn logout(
    user: web::Json<LogoutUser>,
    state: web::Data<AppState>,
) -> impl IntoFuture<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::user::FetchUser {
            username: user.username.to_owned(),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(fetched_user) => Ok(fetched_user),
            Err(e) => {
                return Err(ErrorBadRequest(e));
            }
        })
        .and_then(move |user| {
            state
                .db
                .send(db::token::DeleteToken(user.username))
                .from_err()
                .and_then(|token_response| match token_response {
                    Ok(_token) => Ok(HttpResponse::Ok().finish()),
                    Err(_) => Err(ErrorInternalServerError(
                        "Failed to create session token",
                    )),
                })
        })
}

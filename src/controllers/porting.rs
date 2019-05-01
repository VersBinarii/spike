use actix_web::{web, Error as ActixErr, HttpResponse};
use futures::Future;

use std::error::Error;

use crate::db;
use crate::models::porting::NewPorting;
use crate::AppState;

pub fn list_portings(
    q: web::Query<db::pagination::PaginateQuery>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::porting::FetchPortings {
            page: q.page.unwrap_or(1),
            per_page: q.per_page.unwrap_or(10),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(portings) => Ok(HttpResponse::Ok().json(portings)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn create_new_porting(
    req: web::Json<NewPorting>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::porting::InsertPorting(req.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(new_porting) => Ok(HttpResponse::Ok().json(new_porting)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn update_porting(
    (porting_id, porting_form): (web::Path<i32>, web::Json<NewPorting>),
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::porting::UpdatePorting {
            porting_id: porting_id.into_inner(),
            porting: porting_form.into_inner(),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(updated_porting) => Ok(HttpResponse::Ok().json(updated_porting)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn show_porting(
    porting_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::porting::FetchPorting(porting_id.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(porting) => Ok(HttpResponse::Ok().json(porting)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

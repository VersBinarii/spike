use actix_web::{web, Error as ActixErr, HttpResponse};
use futures::Future;

use std::error::Error;

use crate::db::{self, pagination::PaginateResponse};
use crate::models::{number::Number, Mna, NewMna};
use crate::AppState;

pub fn list_mna(
    q: web::Query<db::pagination::PaginateQuery>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::mna::FetchMnas {
            page: q.page.unwrap_or(1),
            per_page: q.per_page.unwrap_or(10),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(mnas) => Ok(HttpResponse::Ok().json(mnas)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn show_mna(
    mna_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::mna::FetchMna(mna_id.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(mna) => Ok(HttpResponse::Ok().json(mna)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn create_new_mna(
    req: web::Json<NewMna>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::mna::InsertMna(req.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(mna) => Ok(HttpResponse::Ok().json(mna)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn update_mna(
    (mna_id, mna_form): (web::Path<i32>, web::Json<NewMna>),
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::mna::UpdateMna {
            mna_id: mna_id.into_inner(),
            mna: mna_form.into_inner(),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(mna) => Ok(HttpResponse::Ok().json(mna)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

#[derive(Debug, Serialize)]
struct NumbersPerMnaResponse {
    mna: Mna,
    numbers: Vec<Number>,
    pagination: PaginateResponse,
}

pub fn show_numbers_per_mna(
    (mna_id, pagination): (
        web::Path<i32>,
        web::Query<db::pagination::PaginateQuery>,
    ),
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    state
        .db
        .send(db::mna::ShowNumbersMna {
            mna_id: mna_id.into_inner(),
            page,
            per_page,
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(mna) => {
                let (mna_with_numbers, total_pages) = mna;
                let (mut mnas, numbers): (Vec<_>, Vec<_>) =
                    mna_with_numbers.into_iter().unzip();

                match mnas.pop() {
                    Some(mna) => {
                        Ok(HttpResponse::Ok().json(NumbersPerMnaResponse {
                            mna,
                            numbers,
                            pagination: PaginateResponse {
                                page,
                                per_page,
                                total_pages,
                            },
                        }))
                    }
                    None => Ok(HttpResponse::NotFound().json("MNA not found")),
                }
            }
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

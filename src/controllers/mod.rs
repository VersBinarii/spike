use actix::prelude::Addr;
use actix_web::{web, Error as ActixErr, HttpResponse};
use futures::Future;

use std::error::Error;

use crate::db;
use crate::models::*;

pub struct AppState {
    pub db: Addr<db::DbExecutor>,
}

pub fn dummy(
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::number::FetchNumbers)
        .from_err()
        .and_then(move |res| match res {
            Ok(numbers) => Ok(HttpResponse::Ok().json(numbers)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}

pub fn list_numbers(
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::number::FetchNumbers)
        .from_err()
        .and_then(move |res| match res {
            Ok(numbers) => Ok(HttpResponse::Ok().json(numbers)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
}

pub fn create_new_number(
    req: web::Json<NewNumber>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::number::InsertNumber(req.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(new_number) => Ok(HttpResponse::Ok().json(new_number)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn update_number(
    (number_id, number_form): (web::Path<i32>, web::Json<NewNumber>),
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::number::UpdateNumber {
            number_id: number_id.into_inner(),
            number: number_form.into_inner(),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(updated_number) => Ok(HttpResponse::Ok().json(updated_number)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn show_number(
    number_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::number::FetchNumber(number_id.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(number) => Ok(HttpResponse::Ok().json(number)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

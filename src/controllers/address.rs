use actix_web::{web, Error as ActixErr, HttpResponse};
use futures::Future;

use std::error::Error;

use crate::db;
use crate::models::NewAddress;
use crate::AppState;

pub fn show_address(
    address_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::address::FetchAddress(address_id.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(address) => Ok(HttpResponse::Ok().json(address)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn list_addresses(
    q: web::Query<db::pagination::PaginateQuery>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::address::FetchAddresses {
            page: q.page.unwrap_or(1),
            per_page: q.per_page.unwrap_or(10),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(addresses) => Ok(HttpResponse::Ok().json(addresses)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn create_new_address(
    req: web::Json<NewAddress>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::address::InsertAddress(req.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(new_address) => Ok(HttpResponse::Ok().json(new_address)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn update_address(
    (address_id, address): (web::Path<i32>, web::Json<NewAddress>),
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::address::UpdateAddress {
            address_id: address_id.into_inner(),
            address: address.into_inner(),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(updated_address) => Ok(HttpResponse::Ok().json(updated_address)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

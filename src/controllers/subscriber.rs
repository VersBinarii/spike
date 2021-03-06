use actix_web::{web, Error as ActixErr, HttpResponse};
use futures::Future;

use std::error::Error;

use crate::db;
use crate::models::NewSubscriber;
use crate::AppState;

pub fn show_subscriber(
    subscriber_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::subscriber::FetchSubscriber(subscriber_id.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(subscriber) => Ok(HttpResponse::Ok().json(subscriber)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn list_subscribers(
    q: web::Query<db::pagination::PaginateQuery>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::subscriber::FetchSubscribers {
            page: q.page.unwrap_or(1),
            per_page: q.per_page.unwrap_or(10),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(subs) => Ok(HttpResponse::Ok().json(subs)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn create_new_subscriber(
    req: web::Json<NewSubscriber>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::subscriber::InsertSubscriber(req.into_inner()))
        .from_err()
        .and_then(move |res| match res {
            Ok(new_sub) => Ok(HttpResponse::Ok().json(new_sub)),
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

pub fn update_subscriber(
    (subscriber_id, subscriber_form): (
        web::Path<i32>,
        web::Json<NewSubscriber>,
    ),
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ActixErr> {
    state
        .db
        .send(db::subscriber::UpdateSubscriber {
            subscriber_id: subscriber_id.into_inner(),
            subscriber: subscriber_form.into_inner(),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(updated_subscriber) => {
                Ok(HttpResponse::Ok().json(updated_subscriber))
            }
            Err(e) => {
                Ok(HttpResponse::InternalServerError().json(e.description()))
            }
        })
}

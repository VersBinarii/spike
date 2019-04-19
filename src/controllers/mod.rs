pub mod mna;
pub mod number;

use actix::prelude::Addr;
use actix_web::{web, Error as ActixErr, HttpResponse};
use futures::Future;

use crate::db;
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

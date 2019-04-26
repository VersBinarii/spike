pub mod address;
pub mod mna;
pub mod number;
pub mod rsp;
pub mod session;
pub mod subscriber;

use actix::prelude::Addr;
use actix_web::{web, Error as ActixErr, HttpRequest};
use futures::{future::ok, Future};

use crate::db;
pub struct AppState {
    pub db: Addr<db::DbExecutor>,
}

pub fn dummy(
    req: HttpRequest,
    _: web::Data<AppState>,
) -> impl Future<Item = String, Error = ActixErr> {
    ok(format!(
        "I'm a dummy route handler for path: {}",
        req.path()
    ))
}

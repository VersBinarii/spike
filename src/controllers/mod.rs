pub mod mna;
pub mod number;
pub mod rsp;
pub mod session;
pub mod subscriber;

use actix::prelude::Addr;
use actix_web::{web, Error as ActixErr};
use futures::{future::ok, Future};

use crate::db;
pub struct AppState {
    pub db: Addr<db::DbExecutor>,
}

pub fn dummy(
    _: web::Data<AppState>,
) -> impl Future<Item = &'static str, Error = ActixErr> {
    ok("I'm a dummy route handler")
}

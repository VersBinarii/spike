#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix::prelude::SyncArbiter;
use actix_web::{
    middleware::Logger as RequestLogger, web, App, HttpResponse, HttpServer,
};

use crate::controllers::AppState;

mod controllers;
mod db;
mod middleware;
mod models;
mod router;
mod schema;
mod utils;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let runner = actix::System::new("Spike - API server");
    let db_url =
        "postgres://spike_user:spike_user_testing@0.0.0.0:5432/spike_rs";
    let addr = SyncArbiter::start(num_cpus::get(), move || {
        db::DbExecutor::new(db_url)
    });

    // start http server
    HttpServer::new(move || {
        App::new()
            .data(AppState { db: addr.clone() })
            .wrap(RequestLogger::default())
            .wrap(middleware::CheckAuth)
            .configure(router::config_routes)
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind("0.0.0.0:9080")?
    .start();

    runner.run()
}

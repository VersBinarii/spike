#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix::prelude::SyncArbiter;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};

mod controllers;
mod db;
mod models;
mod schema;
mod utils;

use crate::controllers::AppState;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let runner = actix::System::new("Spike - API server");
    let db_url =
        "postgres://spike_user:spike_user_testing@localhost:5432/spike_rs";
    let addr = SyncArbiter::start(num_cpus::get(), move || {
        db::DbExecutor::new(db_url)
    });

    // start http server
    HttpServer::new(move || {
        App::new()
            .data(AppState { db: addr.clone() })
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/numbers")
                    .route(web::get().to_async(controllers::list_numbers))
                    .route(
                        web::post().to_async(controllers::create_new_number),
                    ),
            )
            .service(
                web::resource("/numbers/{id}")
                    .route(web::get().to_async(controllers::show_number))
                    .route(web::put().to_async(controllers::update_number))
                    .route(web::delete().to_async(controllers::dummy)),
            )
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind("0.0.0.0:8080")?
    .start();

    runner.run()
}

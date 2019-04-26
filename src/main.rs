#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix::prelude::SyncArbiter;
use actix_web::{
    middleware::Logger as RequestLogger, web, App, HttpResponse, HttpServer,
};

mod controllers;
mod db;
mod middleware;
mod models;
mod schema;
mod utils;

use crate::controllers::{
    address, mna, number, rsp, session, subscriber, AppState,
};

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
            .service(
                web::resource("/login")
                    .route(web::post().to_async(session::login)),
            )
            .service(
                web::resource("/logout")
                    .route(web::post().to_async(session::logout)),
            )
            .service(
                web::resource("/numbers")
                    .route(web::get().to_async(number::list_numbers))
                    .route(web::post().to_async(number::create_new_number)),
            )
            .service(
                web::resource("/numbers/{id}")
                    .route(web::get().to_async(number::show_number))
                    .route(web::put().to_async(number::update_number))
                    .route(web::delete().to_async(controllers::dummy)),
            )
            .service(
                web::resource("/mna")
                    .route(web::get().to_async(mna::list_mna))
                    .route(web::post().to_async(mna::create_new_mna)),
            )
            .service(
                web::resource("/mna/{id}")
                    .route(web::get().to_async(mna::show_mna))
                    .route(web::put().to_async(mna::update_mna))
                    .route(web::delete().to_async(controllers::dummy)),
            )
            .service(
                web::resource("/mna/{id}/numbers")
                    .route(web::get().to_async(mna::show_numbers_per_mna)),
            )
            .service(
                web::resource("/rsp")
                    .route(web::get().to_async(rsp::list_rsp))
                    .route(web::post().to_async(rsp::create_new_rsp)),
            )
            .service(
                web::resource("/rsp/{id}")
                    .route(web::get().to_async(rsp::show_rsp))
                    .route(web::put().to_async(rsp::update_rsp))
                    .route(web::delete().to_async(controllers::dummy)),
            )
            .service(
                web::resource("/rsp/{id}/subscribers")
                    .route(web::get().to_async(controllers::dummy)),
            )
            .service(
                web::resource("/subscriber")
                    .route(web::get().to_async(subscriber::list_subscribers))
                    .route(
                        web::post().to_async(subscriber::create_new_subscriber),
                    ),
            )
            .service(
                web::resource("/subscriber/{id}")
                    .route(web::get().to_async(subscriber::show_subscriber))
                    .route(web::put().to_async(subscriber::update_subscriber))
                    .route(web::delete().to_async(controllers::dummy)),
            )
            .service(
                web::resource("/address")
                    .route(web::get().to_async(address::list_addresses))
                    .route(web::post().to_async(address::create_new_address)),
            )
            .service(
                web::resource("/address/{id}")
                    .route(web::get().to_async(address::show_address))
                    .route(web::put().to_async(address::update_address))
                    .route(web::delete().to_async(controllers::dummy)),
            )
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind("0.0.0.0:9080")?
    .start();

    runner.run()
}

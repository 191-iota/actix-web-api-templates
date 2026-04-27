use actix_cors::Cors;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware::Logger;
use actix_web::web;
use dotenv::dotenv;
use env_logger::Env;
use log::info;

use self::config::startup;
use self::routes::global_routes;

mod config;
mod error;
mod handler;
mod model;
mod repository;
mod routes;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let address = startup::setup_address();
    let app_state = web::Data::new(startup::init_app_state());

    info!("Running at http://{}:{}", address.0, address.1);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec!["Content-Type"])
                    .max_age(3600),
            )
            .app_data(app_state.clone())
            .configure(global_routes::init_anon_scope)
    })
    .bind(format!("{}:{}", address.0, address.1))?
    .run()
    .await
}
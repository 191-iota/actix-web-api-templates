use actix_cors::Cors;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::middleware::Logger;
use actix_web::web;
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use self::config::startup;
use self::routes::global_routes;
use crate::handler::bookmark_handler::__path_create_bookmark;
use crate::handler::bookmark_handler::__path_list_bookmarks;
use crate::handler::config_handler::__path_health_check;
use crate::model::bookmark::BookmarkRes;
use crate::model::bookmark::CreateBookmarkReq;

mod config;
mod error;
mod handler;
mod model;
mod repository;
mod routes;
mod service;

#[derive(OpenApi)]
#[openapi(
    paths(health_check, create_bookmark, list_bookmarks),
    components(schemas(CreateBookmarkReq, BookmarkRes)),
    tags(
        (name = "config", description = "Configuration endpoints"),
        (name = "bookmark", description = "Bookmark endpoints")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let address = startup::setup_address();
    let app_state = web::Data::new(startup::init_app_state());
    let openapi = ApiDoc::openapi();

    info!("Running at http://{}:{}", address.0, address.1);

    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
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
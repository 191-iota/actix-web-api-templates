use actix_web::web;

use crate::handler::bookmark_handler::create_bookmark;
use crate::handler::bookmark_handler::list_bookmarks;
use crate::handler::config_handler::health_check;

pub fn init_anon_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/health", web::get().to(health_check))
            .route("/api/bookmarks", web::post().to(create_bookmark))
            .route("/api/bookmarks", web::get().to(list_bookmarks)),
    );
}

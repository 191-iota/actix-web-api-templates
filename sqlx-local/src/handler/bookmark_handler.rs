use actix_web::HttpResponse;
use actix_web::web;
use log::error;
use validator::Validate;

use crate::model::app_state::AppState;
use crate::model::bookmark::CreateBookmarkReq;
use crate::repository::bookmark_repository;
use crate::service::url_service;

pub async fn create_bookmark(
    state: web::Data<AppState>,
    body: web::Json<CreateBookmarkReq>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(e);
    }

    let page = match url_service::extract_page_metadata(&state.http_client, &body.url).await {
        Ok(page) => page,
        Err(e) => {
            error!("Failed extracting page metadata: {e}");
            return HttpResponse::BadRequest().body("Failed processing url");
        }
    };

    let res = bookmark_repository::save_bookmark(&state.db, &body.url, &page.host, page.title)
        .await;

    match res {
        Ok(bookmark) => HttpResponse::Ok().json(bookmark),
        Err(e) => {
            error!("Failed creating bookmark: {e}");
            HttpResponse::InternalServerError().body("Failed saving bookmark")
        }
    }
}

pub async fn list_bookmarks(state: web::Data<AppState>) -> HttpResponse {
    match bookmark_repository::list_bookmarks(&state.db).await {
        Ok(bookmarks) => HttpResponse::Ok().json(bookmarks),
        Err(e) => {
            error!("Failed listing bookmarks: {e}");
            HttpResponse::InternalServerError().body("Failed retrieving bookmarks")
        }
    }
}
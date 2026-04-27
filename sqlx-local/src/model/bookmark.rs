use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateBookmarkReq {
    #[validate(url)]
    pub url: String,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct BookmarkRes {
    pub id: i64,
    pub url: String,
    pub host: String,
    pub title: Option<String>,
    pub created_at: String,
}

pub struct NewBookmark {
    pub url: String,
    pub host: String,
    pub title: Option<String>,
}

pub struct PageMetadata {
    pub host: String,
    pub title: Option<String>,
}

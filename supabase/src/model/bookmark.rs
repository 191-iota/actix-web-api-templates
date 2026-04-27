use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBookmarkReq {
    #[validate(url)]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkRes {
    pub id: String,
    pub url: String,
    pub host: String,
    pub title: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug)]
pub struct NewBookmark {
    pub url: String,
    pub host: String,
    pub title: Option<String>,
}

pub struct PageMetadata {
    pub host: String,
    pub title: Option<String>,
}

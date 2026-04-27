use log::error;
use sqlx::SqlitePool;

use crate::error::custom_errors::RepoError;
use crate::model::bookmark::BookmarkRes;
use crate::model::bookmark::NewBookmark;

pub async fn save_bookmark(
    db: &SqlitePool,
    url: &str,
    host: &str,
    title: Option<String>,
) -> Result<BookmarkRes, RepoError> {
    let bookmark = NewBookmark {
        url: url.to_string(),
        host: host.to_string(),
        title,
    };

    sqlx::query_as::<_, BookmarkRes>(
        r#"
        INSERT INTO bookmark (url, host, title)
        VALUES (?1, ?2, ?3)
        RETURNING id, url, host, title, created_at
        "#,
    )
    .bind(bookmark.url)
    .bind(bookmark.host)
    .bind(bookmark.title)
    .fetch_one(db)
    .await
    .map_err(|e| {
        error!("Failed inserting bookmark: {e}");
        RepoError::InsertionError("Failed inserting bookmark".to_string())
    })
}

pub async fn list_bookmarks(db: &SqlitePool) -> Result<Vec<BookmarkRes>, RepoError> {
    sqlx::query_as::<_, BookmarkRes>(
        r#"
        SELECT id, url, host, title, created_at
        FROM bookmark
        ORDER BY id DESC
        "#,
    )
    .fetch_all(db)
    .await
    .map_err(|e| {
        error!("Failed retrieving bookmarks: {e}");
        RepoError::ExtractionError("Failed retrieving bookmarks".to_string())
    })
}

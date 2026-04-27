use log::error;
use serde_json::Value;
use serde_json::json;
use supabase_rs::SupabaseClient;
use uuid::Uuid;

use crate::error::custom_errors::RepoError;
use crate::model::bookmark::BookmarkRes;
use crate::model::bookmark::NewBookmark;

pub async fn save_bookmark(
    db: &SupabaseClient,
    table: &str,
    bookmark: NewBookmark,
) -> Result<BookmarkRes, RepoError> {
    let id = Uuid::new_v4().to_string();

    db.insert(
        table,
        json!({
            "id": id,
            "url": bookmark.url,
            "host": bookmark.host,
            "title": bookmark.title,
        }),
    )
    .await
    .map_err(|e| {
        error!("Failed inserting bookmark: {e}");
        RepoError::InsertionError("Failed inserting bookmark".to_string())
    })?;

    find_bookmark_by_id(db, table, &id).await
}

pub async fn list_bookmarks(
    db: &SupabaseClient,
    table: &str,
) -> Result<Vec<BookmarkRes>, RepoError> {
    let rows = db
        .select(table)
        .columns(vec!["id", "url", "host", "title", "created_at"])
        .execute()
        .await
        .map_err(|e| {
            error!("Failed retrieving bookmarks: {e}");
            RepoError::ExtractionError("Failed retrieving bookmarks".to_string())
        })?;

    serde_json::from_value(Value::Array(rows))
        .map_err(|e| RepoError::ExtractionError(e.to_string()))
}

async fn find_bookmark_by_id(
    db: &SupabaseClient,
    table: &str,
    id: &str,
) -> Result<BookmarkRes, RepoError> {
    let rows = db
        .select(table)
        .eq("id", id)
        .limit(1)
        .execute()
        .await
        .map_err(|e| {
            error!("Failed retrieving bookmark {id}: {e}");
            RepoError::ExtractionError("Failed retrieving bookmark".to_string())
        })?;

    rows.into_iter()
        .next()
        .ok_or_else(|| RepoError::ExtractionError("Bookmark not found".to_string()))
        .and_then(|value| {
            serde_json::from_value(value)
                .map_err(|e| RepoError::ExtractionError(e.to_string()))
        })
}

use sqlx::SqlitePool;

pub struct AppState {
    pub db: SqlitePool,
    pub http_client: reqwest::Client,
}

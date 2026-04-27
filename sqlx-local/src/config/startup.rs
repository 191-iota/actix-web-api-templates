use std::env;

use log::warn;
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

use crate::model::app_state::AppState;

pub fn setup_address() -> (String, String) {
    let host = env::var("HOST").unwrap_or_else(|_| {
        warn!("Could not find HOST env, defaulting to 127.0.0.1");
        "127.0.0.1".to_string()
    });

    let port = env::var("PORT").unwrap_or_else(|_| {
        warn!("Could not find PORT env, defaulting to 8080");
        "8080".to_string()
    });

    (host, port)
}

pub async fn init_app_state() -> AppState {
    let db = init_sqlite_pool().await;

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed running SQLx migrations");

    AppState {
        db,
        http_client: reqwest::Client::new(),
    }
}

async fn init_sqlite_pool() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").expect("Could not find DATABASE_URL");

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed connecting to SQLite")
}
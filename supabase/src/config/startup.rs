use std::env;

use log::warn;
use supabase_rs::SupabaseClient;

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

pub fn init_app_state() -> AppState {
    AppState {
        sb_client: init_supabase_db_client(),
        http_client: reqwest::Client::new(),
        bookmark_table: env::var("BOOKMARK_TABLE").unwrap_or_else(|_| "bookmark".to_string()),
    }
}

fn init_supabase_db_client() -> SupabaseClient {
    SupabaseClient::new(
        env::var("SUPABASE_URL").expect("Could not find SUPABASE_URL"),
        env::var("SUPABASE_API_KEY").expect("Could not find SUPABASE_API_KEY"),
    )
    .expect("Failed initializing Supabase client")
}
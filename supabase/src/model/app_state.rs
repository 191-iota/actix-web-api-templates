use supabase_rs::SupabaseClient;

pub struct AppState {
    pub sb_client: SupabaseClient,
    pub http_client: reqwest::Client,
    pub bookmark_table: String,
}

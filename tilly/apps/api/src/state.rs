use std::{
    env,
    sync::{Arc, Mutex},
};

use crate::models::{hustle::Hustle, transaction::Transaction, user::User};
use crate::services::squad::SquadConfig;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub hustles: Arc<Mutex<Vec<Hustle>>>,
    pub transactions: Arc<Mutex<Vec<Transaction>>>,
    pub http_client: reqwest::Client,
    pub gemini_api_key: Option<String>,
    pub squad_config: SquadConfig,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
            hustles: Arc::new(Mutex::new(Vec::new())),
            transactions: Arc::new(Mutex::new(Vec::new())),
            http_client: reqwest::Client::new(),
            gemini_api_key: env::var("GEMINI_API_KEY").ok(),
            squad_config: SquadConfig {
                base_url: env::var("SQUAD_BASE_URL")
                    .unwrap_or_else(|_| "https://api-d.squadco.com".to_string()),
                secret_key: env::var("SQUAD_SECRET_KEY").ok(),
                business_id: env::var("SQUAD_BUSINESS_ID").ok(),
            },
        }
    }
}

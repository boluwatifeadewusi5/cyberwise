use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub user_id: String,
    pub hustle_id: String,
    pub amount_ngn: u32,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

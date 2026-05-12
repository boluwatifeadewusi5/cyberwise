use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hustle {
    pub id: String,
    pub title: String,
    pub description: String,
    pub location: String,
    pub budget_ngn: u32,
    pub required_skills: Vec<String>,
    pub assigned_user_id: Option<String>,
    pub is_completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateHustleRequest {
    pub title: String,
    pub description: String,
    pub location: String,
    pub budget_ngn: u32,
    pub required_skills: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CompleteHustleRequest {
    pub user_id: String,
}

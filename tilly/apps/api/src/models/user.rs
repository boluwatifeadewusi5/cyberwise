use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub location: String,
    pub skills: Vec<String>,
    pub completed_jobs: u32,
    pub payment_reliability: f32,
    pub customer_reviews: f32,
    pub response_speed: f32,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub location: String,
    pub skills: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct TrustScoreResponse {
    pub user_id: String,
    pub score: f32,
    pub validation_source: String,
    pub notes: String,
}

impl User {
    pub fn trust_score(&self) -> f32 {
        let completed_jobs = (self.completed_jobs.min(20) as f32 / 20.0) * 100.0;
        (0.40 * completed_jobs)
            + (0.25 * self.payment_reliability)
            + (0.20 * self.customer_reviews)
            + (0.15 * self.response_speed)
    }
}

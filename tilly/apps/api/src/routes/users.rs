use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    models::user::{CreateUserRequest, TrustScoreResponse, User},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user).get(list_users))
        .route("/{id}/trust-score", get(get_trust_score))
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, (StatusCode, Json<serde_json::Value>)> {
    if payload.name.trim().is_empty() || payload.location.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "name and location are required"})),
        ));
    }

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        location: payload.location,
        skills: payload.skills,
        completed_jobs: 0,
        payment_reliability: 100.0,
        customer_reviews: 70.0,
        response_speed: 75.0,
    };

    let mut users = state.users.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "state lock poisoned"})),
        )
    })?;

    users.push(user.clone());
    Ok(Json(user))
}

async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, Json<serde_json::Value>)> {
    let users = state.users.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "state lock poisoned"})),
        )
    })?;

    Ok(Json(users.clone()))
}

async fn get_trust_score(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<TrustScoreResponse>, (StatusCode, Json<serde_json::Value>)> {
    let users = state.users.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "state lock poisoned"})),
        )
    })?;

    let user = users
        .iter()
        .find(|user| user.id == id)
        .ok_or((StatusCode::NOT_FOUND, Json(json!({"error": "user not found"}))))?;

    Ok(Json(TrustScoreResponse {
        user_id: user.id.clone(),
        score: user.trust_score(),
    }))
}

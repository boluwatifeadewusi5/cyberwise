use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::post,
};
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{
    models::{
        hustle::{CompleteHustleRequest, CreateHustleRequest, Hustle},
        transaction::Transaction,
    },
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct HustleFilter {
    pub location: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_hustle).get(list_hustles))
        .route("/{id}/complete", post(complete_hustle))
}

async fn create_hustle(
    State(state): State<AppState>,
    Json(payload): Json<CreateHustleRequest>,
) -> Result<Json<Hustle>, (StatusCode, Json<serde_json::Value>)> {
    if payload.title.trim().is_empty() || payload.location.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "title and location are required"})),
        ));
    }

    let hustle = Hustle {
        id: Uuid::new_v4().to_string(),
        title: payload.title,
        description: payload.description,
        location: payload.location,
        budget_ngn: payload.budget_ngn,
        required_skills: payload.required_skills,
        assigned_user_id: None,
        is_completed: false,
    };

    let mut hustles = state.hustles.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "state lock poisoned"})),
        )
    })?;

    hustles.push(hustle.clone());
    Ok(Json(hustle))
}

async fn list_hustles(
    Query(filter): Query<HustleFilter>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Hustle>>, (StatusCode, Json<serde_json::Value>)> {
    let hustles = state.hustles.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "state lock poisoned"})),
        )
    })?;

    let mut result: Vec<Hustle> = hustles.clone();

    if let Some(location) = filter.location {
        let lowercase_location = location.to_lowercase();
        result.retain(|h| h.location.to_lowercase().contains(&lowercase_location));
    }

    Ok(Json(result))
}

async fn complete_hustle(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<CompleteHustleRequest>,
) -> Result<Json<Transaction>, (StatusCode, Json<serde_json::Value>)> {
    {
        let mut users = state.users.lock().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "state lock poisoned"})),
            )
        })?;

        let user = users
            .iter_mut()
            .find(|user| user.id == payload.user_id)
            .ok_or((StatusCode::NOT_FOUND, Json(json!({"error": "user not found"}))))?;

        user.completed_jobs += 1;
        user.customer_reviews = (user.customer_reviews + 2.0).min(100.0);
        user.response_speed = (user.response_speed + 1.0).min(100.0);
    }

    let payout_amount = {
        let mut hustles = state.hustles.lock().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "state lock poisoned"})),
            )
        })?;

        let hustle = hustles
            .iter_mut()
            .find(|h| h.id == id)
            .ok_or((StatusCode::NOT_FOUND, Json(json!({"error": "hustle not found"}))))?;

        hustle.is_completed = true;
        hustle.assigned_user_id = Some(payload.user_id.clone());
        hustle.budget_ngn
    };

    let transaction = Transaction {
        id: Uuid::new_v4().to_string(),
        user_id: payload.user_id,
        hustle_id: id,
        amount_ngn: payout_amount,
        status: "paid_instantly_via_squad".to_string(),
        created_at: Utc::now(),
    };

    let mut transactions = state.transactions.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "state lock poisoned"})),
        )
    })?;
    transactions.push(transaction.clone());

    Ok(Json(transaction))
}

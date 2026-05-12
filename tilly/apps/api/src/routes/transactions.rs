use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;
use serde_json::json;

use crate::{models::transaction::Transaction, state::AppState};

#[derive(Debug, Deserialize)]
struct TransactionFilter {
    user_id: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list_transactions))
}

async fn list_transactions(
    Query(filter): Query<TransactionFilter>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, Json<serde_json::Value>)> {
    let transactions = state.transactions.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "state lock poisoned"})),
        )
    })?;

    let mut result = transactions.clone();

    if let Some(user_id) = filter.user_id {
        result.retain(|t| t.user_id == user_id);
    }

    Ok(Json(result))
}

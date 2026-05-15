use reqwest::Client;
use serde_json::{Value, json};

use crate::models::user::User;

#[derive(Debug, Clone)]
pub struct GeminiValidation {
    pub validated_score: f32,
    pub feedback: String,
    pub source: String,
}

pub async fn validate_user_trust_score(
    http_client: &Client,
    api_key: Option<&str>,
    user: &User,
    calculated_score: f32,
) -> Result<GeminiValidation, String> {
    let Some(api_key) = api_key else {
        return Ok(GeminiValidation {
            validated_score: calculated_score,
            feedback: "Gemini API key is not configured. Returned local weighted score.".to_string(),
            source: "local_fallback".to_string(),
        });
    };

    let prompt = format!(
        "You are validating a youth worker trust score for a gigs marketplace. Return STRICT JSON only with keys validated_score (0-100 float) and feedback (short sentence). User data: name={}, completed_jobs={}, payment_reliability={}, customer_reviews={}, response_speed={}, local_score={}",
        user.name,
        user.completed_jobs,
        user.payment_reliability,
        user.customer_reviews,
        user.response_speed,
        calculated_score
    );

    let request_body = json!({
        "contents": [{
            "parts": [{ "text": prompt }]
        }],
        "generationConfig": {
            "temperature": 0.2,
            "responseMimeType": "application/json"
        }
    });

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={api_key}"
    );

    let response = http_client
        .post(url)
        .json(&request_body)
        .send()
        .await
        .map_err(|error| format!("Gemini request failed: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "failed to decode Gemini error body".to_string());
        return Err(format!("Gemini returned status {status}: {body}"));
    }

    let body: Value = response
        .json()
        .await
        .map_err(|error| format!("Gemini response JSON decode failed: {error}"))?;

    let text = body
        .get("candidates")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("content"))
        .and_then(|c| c.get("parts"))
        .and_then(|p| p.get(0))
        .and_then(|p| p.get("text"))
        .and_then(Value::as_str)
        .ok_or("Gemini response missing content text")?;

    let parsed: Value = serde_json::from_str(text)
        .map_err(|error| format!("Gemini content is not valid JSON: {error}"))?;

    let validated_score = parsed
        .get("validated_score")
        .and_then(Value::as_f64)
        .map(|value| value.clamp(0.0, 100.0) as f32)
        .unwrap_or(calculated_score);

    let feedback = parsed
        .get("feedback")
        .and_then(Value::as_str)
        .unwrap_or("Gemini validated trust score.")
        .to_string();

    Ok(GeminiValidation {
        validated_score,
        feedback,
        source: "gemini".to_string(),
    })
}

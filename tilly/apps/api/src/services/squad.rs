use reqwest::Client;
use serde_json::{Value, json};

#[derive(Clone)]
pub struct SquadConfig {
    pub base_url: String,
    pub secret_key: Option<String>,
    pub business_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SquadDisbursement {
    pub status: String,
    pub provider_reference: Option<String>,
}

pub async fn disburse_payment(
    http_client: &Client,
    config: &SquadConfig,
    user_id: &str,
    hustle_id: &str,
    amount_ngn: u32,
) -> Result<SquadDisbursement, String> {
    let Some(secret_key) = config.secret_key.as_deref() else {
        return Ok(SquadDisbursement {
            status: "paid_simulated_local".to_string(),
            provider_reference: None,
        });
    };

    let request_body = json!({
        "amount": amount_ngn,
        "currency": "NGN",
        "reference": format!("tilly-{}-{}", user_id, hustle_id),
        "narration": "TILLY instant gig payout",
        "business_id": config.business_id,
        "beneficiary": {
            "name": format!("user-{user_id}"),
            "account_number": "0000000000",
            "bank_code": "000"
        }
    });

    let url = format!("{}/v1/payout", config.base_url.trim_end_matches('/'));

    let response = http_client
        .post(url)
        .header("Authorization", format!("Bearer {secret_key}"))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|error| format!("Squad request failed: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "failed to decode Squad error body".to_string());
        return Err(format!("Squad returned status {status}: {body}"));
    }

    let payload: Value = response
        .json()
        .await
        .map_err(|error| format!("Squad response JSON decode failed: {error}"))?;

    let provider_reference = payload
        .get("data")
        .and_then(|data| data.get("reference").or_else(|| data.get("transaction_ref")))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);

    Ok(SquadDisbursement {
        status: "paid_instantly_via_squad".to_string(),
        provider_reference,
    })
}

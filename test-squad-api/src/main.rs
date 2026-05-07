use reqwest::Client;
use serde_json::json;
use dotenvy::dotenv;
use std::env;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok(); // loads .env file

    let secret_key = env::var("SQUAD_SECRET_KEY")?;
    let public_key = env::var("SQUAD_PUBLIC_KEY")?;

    let client = Client::new();
    
    let response = client
        .post("https://sandbox-api-d.squadco.com/transaction/initiate")
        .header("Authorization", secret_key)
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": "semajayi1234@gmail.com",
            "currency": "NGN",
            "initiate_type": "inline",
            "callback_url": "https://www.linkedin.com/",
            "amount": "20000"
        }))
        .send()
        .await?;

    let text = response.text().await?;

    println!("{}", text);

    attempt_transaction().await;
    Ok(())
}

async fn attempt_transaction() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok(); // loads .env file

    let secret_key = env::var("SQUAD_SECRET_KEY")?;
    let public_key = env::var("SQUAD_PUBLIC_KEY")?;
    
    let client = Client::new();

    let response = client
        .post("https://sandbox-api-d.squadco.com/virtual-account/simulate/payment")
        .header("Authorization", secret_key)
        .json(&json!({
            "virtual_account_number": "9279755518",
            "amount": "20000"
        }))
        .send()
        .await?;

    println!("SIMULATION RESPONSE:\n{}", response.text().await?);

    Ok(())
}
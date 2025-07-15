use reqwest::header::{ACCEPT, HeaderMap};
use serde_json::Value;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse()?);
    let client = reqwest::Client::new();
    let res = client
        .get("https://horizon-testnet.stellar.org/trades")
        .headers(headers)
        .send()
        .await?;
    let body = res.text().await?;
    let json: Value = serde_json::from_str(&body)?;
    let mut addresses = HashSet::new();
    if let Some(records) = json["_embedded"]["records"].as_array() {
        for record in records {
            if let Some(base) = record.get("base_account") {
                if base.is_string() {
                    addresses.insert(base.as_str().unwrap().to_string());
                }
            }
            if let Some(counter) = record.get("counter_account") {
                if counter.is_string() {
                    addresses.insert(counter.as_str().unwrap().to_string());
                }
            }
        }
    }
    let output: Vec<_> = addresses
        .into_iter()
        .map(|address| {
            serde_json::json!({
                "address": address,
                "amount": 1000000000
            })
        })
        .collect();
    let mut file = File::create("sdex-traders.json")?;
    file.write_all(serde_json::to_string_pretty(&output)?.as_bytes())?;
    println!("Wrote {} addresses to sdex-traders.json", output.len());
    Ok(())
}

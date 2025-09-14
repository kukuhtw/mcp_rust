// src/fetch.rs

use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

pub async fn fetch_one(
    client: &Client,
    base_url: &str,
    endpoint: &str,
    params: &HashMap<String, String>,
) -> Result<Value> {
    let mut req = client.get(format!("{base_url}{endpoint}"));
    if !params.is_empty() {
        req = req.query(params);
    }
    let resp = req.send().await?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        tracing::error!(%endpoint, %status, body = %body, "fetch failed");
        anyhow::bail!("fetch {endpoint} failed {status}: {body}");
    }
    Ok(resp.json::<Value>().await?)
}

pub async fn fetch_join(
    client: &Client,
    base_url: &str,
    endpoints: &[String],
    params: &HashMap<String, String>,
) -> Result<Value> {
    let mut joined = Vec::<serde_json::Value>::with_capacity(endpoints.len());
    for ep in endpoints {
        let data = fetch_one(client, base_url, ep, params)
            .await
            .unwrap_or_else(|e| serde_json::json!({ "error": e.to_string() }));
        joined.push(serde_json::json!({ "endpoint": ep, "data": data }));
    }
    Ok(serde_json::json!({ "results": joined }))
}

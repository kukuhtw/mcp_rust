// backend/src/intent.rs

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;

/// What we want to return to the router
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IntentResult {
    pub intent: String,
    pub endpoints: Vec<String>,
}

/// Extract the JSON string produced by the Responses API (json_schema)
/// Robust to minor shape changes: prefer output[0].content[0].text
fn parse_openai_payload(resp: &serde_json::Value) -> Result<serde_json::Value> {
    // Typical shape:
    // resp.output[0].content[0].text = "{\"intent\":\"...\",\"endpoints\":[...]}"
    let text = resp["output"]
        .get(0)
        .and_then(|o| o["content"].get(0))
        .and_then(|c| c["text"].as_str())
        .ok_or_else(|| anyhow!("OpenAI response missing output[0].content[0].text"))?;

    let parsed: serde_json::Value = serde_json::from_str(text)
        .map_err(|e| anyhow!("Failed to parse model JSON: {e}; text={text}"))?;
    Ok(parsed)
}

/// Call OpenAI Responses API with json_schema and parse out IntentResult
pub async fn detect_intent(cfg: &Config, user_query: &str) -> Result<IntentResult> {
    let client = Client::new();

    // System prompt for intent detection
    let system_prompt = std::env::var("SYSTEM_PROMPT").unwrap_or_else(|_| {
        "You are an MCP intent router. Classify the user query into an intent \
         and list the API endpoints to call. Output must be JSON conforming to the schema."
            .to_string()
    });

    // JSON schema describing the classification output
    let schema = serde_json::json!({
        "name": "intent_schema",
        "schema": {
            "type": "object",
            "properties": {
                "intent": { "type": "string" },
                "endpoints": {
                    "type": "array",
                    "items": { "type": "string" },
                    "minItems": 1
                }
            },
            "required": ["intent", "endpoints"],
            "additionalProperties": false
        }
    });

    // Build Responses API request
    let base = std::env::var("OPENAI_BASE_URL")
        .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    let body = serde_json::json!({
        "model": cfg.model,
        "input": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": format!("USER QUERY: {}", user_query) }
        ],
        "response_format": { "type": "json_schema", "json_schema": schema }
    });

    let resp_val = client
        .post(format!("{base}/responses"))
        .bearer_auth(&cfg.openai_api_key)
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json::<serde_json::Value>()
        .await?;

    let parsed = parse_openai_payload(&resp_val)?;
    // Convert into our struct
    let out: IntentResult = serde_json::from_value(parsed)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_openai_payload_ok() {
        // Simulate Responses API shape
        let fake = serde_json::json!({
            "output": [{
                "content": [{
                    "type": "output_text",
                    "text": r#"{"intent":"deploy_troubleshoot","endpoints":["/api/gitlab-ci","/api/runtime-logs"]}"#
                }]
            }]
        });
        let parsed = parse_openai_payload(&fake).unwrap();
        let ir: IntentResult = serde_json::from_value(parsed).unwrap();
        assert_eq!(ir.intent, "deploy_troubleshoot");
        assert_eq!(ir.endpoints, vec!["/api/gitlab-ci", "/api/runtime-logs"]);
    }

    #[test]
    fn test_parse_openai_payload_missing_text() {
        let bad = serde_json::json!({ "output": [{ "content": [{}] }] });
        let err = parse_openai_payload(&bad).unwrap_err();
        assert!(format!("{err}").contains("missing"));
    }

    #[test]
    fn test_extra_field_is_ignored_by_serde() {
        // serde (by default) ignores unknown fields
        let data = serde_json::json!({
            "intent": "db_performance",
            "endpoints": ["/api/db-perf"],
            "extra": "ignored"
        });
        let parsed: IntentResult = serde_json::from_value(data).unwrap();
        assert_eq!(parsed.intent, "db_performance");
        assert_eq!(parsed.endpoints, vec!["/api/db-perf"]);
    }
}

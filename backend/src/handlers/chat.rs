// backend/src/handlers/chat.rs

// backend/src/handlers/chat.rs

use crate::config::Config;
use crate::fetch::{fetch_join, fetch_one};
use crate::mcp::{intent_prompt, parse_or_fallback};

use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::sse::{Event, KeepAlive, Sse},
    Json,
};
use futures_util::{Stream, TryStreamExt};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::time::Duration;

/* ------------------------- Types ------------------------- */

#[derive(Deserialize, Debug, Clone)]
pub struct ChatRequest {
    pub text: String,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub tz: Option<String>,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub reply: String,
}

/* ------------------------- Helpers ------------------------- */

fn internal_base_url(headers_opt: Option<&HeaderMap>) -> String {
    // 1) Use ENV if present
    if let Ok(v) = std::env::var("SELF_BASE_URL") {
        if !v.trim().is_empty() {
            return v;
        }
    }
    // 2) Fallback to Host header
    let host = headers_opt
        .and_then(|h| h.get("host"))
        .and_then(|v| v.to_str().ok())
        .unwrap_or("127.0.0.1:8080");
    format!("http://{host}")
}

fn build_client(timeout: Duration) -> Result<reqwest::Client, axum::Error> {
    let mut b = reqwest::Client::builder()
        .timeout(timeout)
        .connect_timeout(Duration::from_secs(10))
        .pool_idle_timeout(Duration::from_secs(90))
        .tcp_keepalive(Some(Duration::from_secs(30)))
        .user_agent("smrt-mcp-backend/0.1 (+github.com/your-org)");

    // proxy (biarkan seperti sebelumnya)
    if let Ok(p) = std::env::var("HTTPS_PROXY").or_else(|_| std::env::var("https_proxy")) {
        if let Ok(proxy) = reqwest::Proxy::https(&p) {
            b = b.proxy(proxy);
        }
    }
    if let Ok(p) = std::env::var("HTTP_PROXY").or_else(|_| std::env::var("http_proxy")) {
        if let Ok(proxy) = reqwest::Proxy::http(&p) {
            b = b.proxy(proxy);
        }
    }

    // (opsional) jika butuh: paksa HTTP/1.1 untuk menghindari h2 handshake isu
    if std::env::var("FORCE_HTTP1").ok().as_deref() == Some("1") {
        b = b.http1_only();
    }

    b.build().map_err(to_axum_error)
}

/// Client without proxy — for internal loopback calls
fn build_client_no_proxy(timeout: Duration) -> Result<reqwest::Client, axum::Error> {
    reqwest::Client::builder()
        .timeout(timeout)
        .no_proxy()
        .build()
        .map_err(to_axum_error)
}

fn internal_error<E: std::fmt::Display>(e: E) -> (axum::http::StatusCode, String) {
    tracing::error!("internal error: {e}");
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

fn to_axum_error<E>(e: E) -> axum::Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    let mut src: &(dyn std::error::Error + 'static) = &e;
    tracing::error!("stream error: {}", src);
    while let Some(cause) = src.source() {
        tracing::error!("  caused by: {}", cause);
        src = cause;
    }
    axum::Error::new(e)
}

/* ------------------------- Non-stream ------------------------- */

#[tracing::instrument(skip(_pool, payload), fields(text = %payload.text))]
pub async fn chat_handler(
    State(_pool): State<MySqlPool>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (axum::http::StatusCode, String)> {
    let cfg = Config::from_env();

    // Upstream (OpenAI): may proxy
    let client_up = build_client(Duration::from_secs(60)).map_err(internal_error)?;
    // Internal fetch: no proxy
    let client_in = build_client_no_proxy(Duration::from_secs(15)).map_err(internal_error)?;

    // ===== 1) Planner =====
    let plan_prompt = intent_prompt(&cfg.system_prompt, &payload.text);
    let plan_req = serde_json::json!({
        "model": &cfg.model,
        "messages": [
            { "role": "system", "content": "Return JSON only. No prose." },
            { "role": "user",   "content": plan_prompt }
        ],
        "temperature": 0.0
    });

    let plan_txt = {
        let r = client_up
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&cfg.openai_api_key)
            .json(&plan_req)
            .send()
            .await
            .map_err(internal_error)?;
        let status = r.status();
        if !status.is_success() {
            let body = r.text().await.unwrap_or_default();
            tracing::warn!(%status, body = %body, "planner failed; fallback heuristic");
            String::new()
        } else {
            #[derive(Deserialize)]
            struct Choice {
                message: MsgOut,
            }
            #[derive(Deserialize)]
            struct MsgOut {
                content: String,
            }
            #[derive(Deserialize)]
            struct PlanResp {
                choices: Vec<Choice>,
            }
            let pr: PlanResp = r.json().await.map_err(internal_error)?;
            pr.choices
                .get(0)
                .map(|c| c.message.content.clone())
                .unwrap_or_default()
        }
    };

    let mut plan = parse_or_fallback(&plan_txt, &payload.text);

    // Inject params dari payload
    if let Some(df) = &payload.date_from {
        plan.params.insert("date_from".into(), df.clone());
    }
    if let Some(dt) = &payload.date_to {
        plan.params.insert("date_to".into(), dt.clone());
    }
    if let Some(tz) = &payload.tz {
        plan.params.insert("tz".into(), tz.clone());
    }

    // Fallback regex untuk service
    if !plan.params.contains_key("service") {
        let t = payload.text.to_lowercase();
        let regex_guess = Regex::new(r"([a-z0-9\-]+)\s+service").ok().and_then(|re| {
            re.captures(&t)
                .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
        });
        let simple_guess = if t.contains("payments") || t.contains("payment") {
            Some("payments".to_string())
        } else if t.contains("auth") {
            Some("auth-service".to_string())
        } else if t.contains("orders") || t.contains("order") {
            Some("orders".to_string())
        } else {
            None
        };
        if let Some(svc) = simple_guess.or(regex_guess) {
            plan.params.insert("service".into(), svc);
        }
    }

    tracing::info!(?plan, "🧭 router plan");

    // ===== 2) Fetch & Join (no-proxy) =====
    let base_url = internal_base_url(None);
    let joined = fetch_join(&client_in, &base_url, &plan.endpoints, &plan.params)
        .await
        .map_err(|e| internal_error(format!("join error: {e}")))?;

    // ===== 3) Final answer =====
    #[derive(Serialize)]
    struct Msg<'a> {
        role: &'a str,
        content: &'a str,
    }
    #[derive(Serialize)]
    struct Req<'a> {
        model: &'a str,
        messages: Vec<Msg<'a>>,
        temperature: f32,
    }
    #[derive(Deserialize)]
    struct Choice {
        message: MsgOut,
    }
    #[derive(Deserialize)]
    struct MsgOut {
        role: String,
        content: String,
    }
    #[derive(Deserialize)]
    struct Resp {
        choices: Vec<Choice>,
    }

    let context = serde_json::to_string_pretty(&joined).unwrap_or_else(|_| "{}".into());
    let user_full = format!("Question: {}\n\nJoined data:\n{}", payload.text, context);

    let body = Req {
        model: &cfg.model,
        messages: vec![
            Msg {
                role: "system",
                content: &cfg.system_prompt,
            },
            Msg {
                role: "user",
                content: &user_full,
            },
        ],
        temperature: 0.2,
    };

    let resp = client_up
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(cfg.openai_api_key)
        .json(&body)
        .send()
        .await
        .map_err(internal_error)?;

    if !resp.status().is_success() {
        let status = resp.status();
        let s = resp.text().await.unwrap_or_default();
        #[derive(Deserialize)]
        struct OaiErr {
            error: OaiErrInner,
        }
        #[derive(Deserialize)]
        struct OaiErrInner {
            message: String,
            r#type: String,
            code: Option<String>,
        }
        let msg = serde_json::from_str::<OaiErr>(&s)
            .map(|e| {
                format!(
                    "OpenAI error: {} (type={}, code={:?})",
                    e.error.message, e.error.r#type, e.error.code
                )
            })
            .unwrap_or_else(|_| format!("OpenAI error {status}: {s}"));
        return Err((axum::http::StatusCode::BAD_GATEWAY, msg));
    }

    let parsed: Resp = resp.json().await.map_err(internal_error)?;
    let content = parsed
        .choices
        .get(0)
        .map(|c| c.message.content.clone())
        .unwrap_or_else(|| "No content".to_string());

    Ok(Json(ChatResponse { reply: content }))
}

/* ------------------------- Stream (SSE) ------------------------- */

#[tracing::instrument(skip(_pool, headers, q))]
pub async fn chat_stream_handler(
    State(_pool): State<MySqlPool>,
    headers: HeaderMap,
    Query(q): Query<ChatRequest>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let req_id = headers
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("no-request-id")
        .to_string();

    let cfg = Config::from_env();
    let model = cfg.model.clone();
    let system_prompt = cfg.system_prompt.clone();
    let user_text = q.text.clone();

    let stream = async_stream::try_stream! {
            // received
            yield Event::default().event("received").id(req_id.clone()).data(user_text.clone());

            if cfg.openai_api_key.trim().is_empty() {
                yield Event::default().event("token").id(req_id.clone()).data("(missing OPENAI_API_KEY)");
                yield Event::default().event("done").id(req_id.clone()).data("done");
                return;
            }

            // Upstream client (may proxy) & internal client (no proxy)
            let client_up = build_client(Duration::from_secs(60))?;
            let client_in = build_client_no_proxy(Duration::from_secs(15))?;

            // llm_start(plan)
            yield Event::default().event("llm_start").id(req_id.clone()).data("plan");

            // Planner
            let plan_prompt_str = intent_prompt(&system_prompt, &user_text);
            let plan_body = serde_json::json!({
                "model": model,
                "messages": [
                    { "role": "system", "content": "Return JSON only. No prose." },
                    { "role": "user",   "content": plan_prompt_str }
                ],
                "temperature": 0.0
            });

            let plan_res = client_up
                .post("https://api.openai.com/v1/chat/completions")
                .bearer_auth(&cfg.openai_api_key)
                .json(&plan_body)
                .send()
                .await;

            let plan_text = match plan_res {
                Ok(rsp) if rsp.status().is_success() => {
                    #[derive(Deserialize)] struct Choice { message: MsgOut }
                    #[derive(Deserialize)] struct MsgOut { content: String }
                    #[derive(Deserialize)] struct PlanResp { choices: Vec<Choice> }
                    let pr: PlanResp = rsp.json().await.map_err(to_axum_error)?;
                    pr.choices.get(0).map(|c| c.message.content.clone()).unwrap_or_default()
                },
                Ok(rsp) => {
                    let st = rsp.status();
                    let body = rsp.text().await.unwrap_or_default();
                    tracing::warn!(%st, body = %body, "planner non-200; using heuristic");
                    String::new()
                }
                Err(e) => {
                    tracing::warn!("planner request failed: {e}; using heuristic");
                    String::new()
                }
            };

            // build plan (fallback → heuristic)
            let mut plan = parse_or_fallback(&plan_text, &user_text);
            if let Some(df) = &q.date_from { plan.params.insert("date_from".into(), df.clone()); }
            if let Some(dt) = &q.date_to   { plan.params.insert("date_to".into(),   dt.clone()); }
            if let Some(tz) = &q.tz        { plan.params.insert("tz".into(),        tz.clone()); }

            // Fallback regex service
            if !plan.params.contains_key("service") {
                let t = user_text.to_lowercase();
                let regex_guess = Regex::new(r"([a-z0-9\-]+)\s+service").ok()
                    .and_then(|re| re.captures(&t).and_then(|c| c.get(1).map(|m| m.as_str().to_string())));
                let simple_guess = if t.contains("payments") || t.contains("payment") {
                    Some("payments".to_string())
                } else if t.contains("auth") {
                    Some("auth-service".to_string())
                } else if t.contains("orders") || t.contains("order") {
                    Some("orders".to_string())
                } else { None };
                if let Some(svc) = simple_guess.or(regex_guess) {
                    plan.params.insert("service".into(), svc);
                }
            }

            // route_planned
            let planned_json = serde_json::to_string(&plan).unwrap_or_else(|_| "{}".into());
            yield Event::default().event("route_planned").id(req_id.clone()).data(planned_json.clone());

            // fetch_progress (internal, no proxy)
            let base_url = internal_base_url(Some(&headers));
            let mut joined = Vec::<serde_json::Value>::with_capacity(plan.endpoints.len());

            for ep in &plan.endpoints {
                yield Event::default().event("fetch_progress").id(req_id.clone()).data(
                    serde_json::json!({ "endpoint": ep, "status": "start" }).to_string()
                );

                let res = fetch_one(&client_in, &base_url, ep, &plan.params).await;

                let (status, data) = match res {
                    Ok(v) => ("ok", v),
                    Err(e) => ("error", serde_json::json!({ "error": e.to_string() })),
                };

                yield Event::default().event("fetch_progress").id(req_id.clone()).data(
                    serde_json::json!({ "endpoint": ep, "status": status }).to_string()
                );

                joined.push(serde_json::json!({ "endpoint": ep, "data": data }));
            }

            let joined_json = serde_json::json!({ "results": joined });
            let joined_pretty = serde_json::to_string_pretty(&joined_json).unwrap_or_else(|_| "{}".into());

            // joined
            yield Event::default().event("joined").id(req_id.clone()).data(joined_pretty.clone());

            // short-circuit if all endpoints errored
            let all_err = joined_json["results"].as_array().map(|arr| arr.iter().all(|it| it["data"].get("error").is_some())).unwrap_or(false);
            if all_err {
                let first_err = joined_json["results"][0]["data"]["error"].as_str().unwrap_or("unknown error");
                let hint = "Hint: ensure params.service is set (e.g. payments) and backend can reach /api/runtime-logs.";
                let msg = format!("(fetch error) {first_err}\n{hint}");
                yield Event::default().event("token").id(req_id.clone()).data(msg);
                yield Event::default().event("done").id(req_id.clone()).data("done");
                return;
            }

            // llm_start(answer)
            yield Event::default().event("llm_start").id(req_id.clone()).data("answer");

            // Final stream call (upstream)
            let final_prompt = serde_json::json!({
                "model": model,
                "stream": true,
                "messages": [
                    { "role": "system", "content": system_prompt },
                    { "role": "user",   "content": format!("Question: {}\n\nJoined data:\n{}", user_text, joined_pretty) }
                ],
                "temperature": 0.2
            });

           let send_res = client_up
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(cfg.openai_api_key.clone())
        .json(&final_prompt)
        .send()
        .await;

        // === retry sekali kalau error kirim (transient) ===
    let send_res = match send_res {
        Ok(r) => Ok(r),
        Err(e1) => {
            tracing::warn!("openai send error (first try): {e1}; retrying once...");
            tokio::time::sleep(Duration::from_millis(250)).await;
            client_up
                .post("https://api.openai.com/v1/chat/completions")
                .bearer_auth(cfg.openai_api_key.clone())
                .json(&final_prompt)
                .send()
                .await
        }
    };

            // === Fallback jika pengiriman upstream gagal ===
            let resp = match send_res {
                Ok(r) => r,
                Err(e) => {
                    // 🔁 Local fallback renderer (no LLM)
                    let logs = joined_json["results"]
                        .get(0).and_then(|x| x.get("data"))
                        .and_then(|d| d.get("logs")).and_then(|l| l.as_array())
                        .cloned().unwrap_or_default();

                    let mut counts = std::collections::BTreeMap::<String, usize>::new();
                    for item in &logs {
                        let lvl = item.get("level").and_then(|v| v.as_str()).unwrap_or("UNKNOWN").to_string();
                        *counts.entry(lvl).or_insert(0) += 1;
                    }

                    let service = joined_json["results"]
                        .get(0).and_then(|x| x.get("data")).and_then(|d| d.get("service"))
                        .and_then(|s| s.as_str()).unwrap_or("unknown");
                    let tz = joined_json["results"]
                        .get(0).and_then(|x| x.get("data")).and_then(|d| d.get("tz"))
                        .and_then(|s| s.as_str()).unwrap_or("UTC");
                    let checked_at = joined_json["results"]
                        .get(0).and_then(|x| x.get("data")).and_then(|d| d.get("checked_at"))
                        .and_then(|s| s.as_str()).unwrap_or("-");

                    let header = format!("Runtime logs (service={service}, tz={tz}) — checked_at={checked_at}\n");
                    yield Event::default().event("token").id(req_id.clone()).data(header);

                    for (lvl, c) in counts {
                        let line = format!("• {lvl}: {c}\n");
                        yield Event::default().event("token").id(req_id.clone()).data(line);
                    }

                    let tail_n = 10usize;
                    let start = logs.len().saturating_sub(tail_n);
                    if !logs.is_empty() {
                        yield Event::default().event("token").id(req_id.clone()).data("\nLast lines:\n");
                    }
                    for item in logs.iter().skip(start) {
                        let ts = item.get("ts").and_then(|v| v.as_str()).unwrap_or("-");
                        let lvl = item.get("level").and_then(|v| v.as_str()).unwrap_or("-");
                        let msg = item.get("message").and_then(|v| v.as_str()).unwrap_or("-");
                        let line = format!("[{ts}] {lvl}: {msg}\n");
                        yield Event::default().event("token").id(req_id.clone()).data(line);
                    }

                    let footer = format!("\n(note) LLM formatting skipped: {e}\n");
                    yield Event::default().event("token").id(req_id.clone()).data(footer);
                    yield Event::default().event("done").id(req_id.clone()).data("done");
                    return;
                }
            };

            // === Fallback jika upstream balas non-200 ===
            if !resp.status().is_success() {
                let status = resp.status();
                let s = resp.text().await.unwrap_or_default();
                #[derive(Deserialize)] struct OaiErr { error: OaiErrInner }
                #[derive(Deserialize)] struct OaiErrInner { message: String, r#type: String, code: Option<String> }
                let msg = match serde_json::from_str::<OaiErr>(&s) {
                    Ok(e) => format!("OpenAI error: {} (type={}, code={:?})", e.error.message, e.error.r#type, e.error.code),
                    Err(_) => format!("OpenAI error {status}: {s}"),
                };

                let logs = joined_json["results"]
                    .get(0).and_then(|x| x.get("data"))
                    .and_then(|d| d.get("logs")).and_then(|l| l.as_array())
                    .cloned().unwrap_or_default();

                let mut counts = std::collections::BTreeMap::<String, usize>::new();
                for item in &logs {
                    let lvl = item.get("level").and_then(|v| v.as_str()).unwrap_or("UNKNOWN").to_string();
                    *counts.entry(lvl).or_insert(0) += 1;
                }

                yield Event::default().event("token").id(req_id.clone()).data("LLM formatting unavailable; showing raw summary:\n");
                for (lvl, c) in counts {
                    yield Event::default().event("token").id(req_id.clone()).data(format!("• {lvl}: {c}\n"));
                }
                let start = logs.len().saturating_sub(10);
                if !logs.is_empty() {
                    yield Event::default().event("token").id(req_id.clone()).data("\nLast lines:\n");
                }
                for item in logs.iter().skip(start) {
                    let ts = item.get("ts").and_then(|v| v.as_str()).unwrap_or("-");
                    let lvl = item.get("level").and_then(|v| v.as_str()).unwrap_or("-");
                    let msg_line = item.get("message").and_then(|v| v.as_str()).unwrap_or("-");
                    yield Event::default().event("token").id(req_id.clone()).data(format!("[{ts}] {lvl}: {msg_line}\n"));
                }
                yield Event::default().event("token").id(req_id.clone()).data(format!("\n(note) {msg}\n"));
                yield Event::default().event("done").id(req_id.clone()).data("done");
                return;
            }

            // stream tokens
            let mut lines = resp.bytes_stream();
            let mut buf = Vec::<u8>::new();

            while let Some(chunk) = lines.try_next().await.map_err(to_axum_error)? {
                buf.extend_from_slice(&chunk);

                loop {
                    if let Some(pos) = buf.iter().position(|&b| b == b'\n') {
                        let line_bytes = buf.drain(..=pos).collect::<Vec<u8>>();
                        let line = String::from_utf8_lossy(&line_bytes).trim().to_string();
                        if line.is_empty() { continue; }
                        if !line.starts_with("data:") { continue; }

                        let data = line.trim_start_matches("data:").trim().to_string();
                        if data == "[DONE]" { break; }

                        #[derive(Deserialize, Debug)] struct StreamDelta { content: Option<String> }
                        #[derive(Deserialize, Debug)] struct StreamChoice { delta: StreamDelta, finish_reason: Option<String> }
                        #[derive(Deserialize, Debug)] struct StreamResp { choices: Vec<StreamChoice> }

                        if let Ok(sr) = serde_json::from_str::<StreamResp>(&data) {
                            if let Some(choice) = sr.choices.get(0) {
                                if let Some(token) = &choice.delta.content {
                                    yield Event::default().event("token").id(req_id.clone()).data(token.clone());
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
            }

            yield Event::default().event("done").id(req_id.clone()).data("done");
        };

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("💓"),
    )
}

/* ------------------------- Ping OpenAI (buat router.rs) ------------------------- */

#[tracing::instrument]
pub async fn openai_ping_handler() -> Result<String, (axum::http::StatusCode, String)> {
    let cfg = Config::from_env();
    let client = build_client(Duration::from_secs(20)).map_err(internal_error)?;
    let resp = client
        .get("https://api.openai.com/v1/models")
        .bearer_auth(cfg.openai_api_key)
        .send()
        .await
        .map_err(internal_error)?;
    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();
    let preview = &body[..body.len().min(400)];
    Ok(format!("status={status}; body={preview}"))
}

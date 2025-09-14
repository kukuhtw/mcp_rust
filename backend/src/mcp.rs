// src/mcp.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterPlan {
    pub intent: String,
    pub endpoints: Vec<String>,
    #[serde(default)]
    pub params: HashMap<String, String>,
}

impl RouterPlan {
    pub fn new(intent: impl Into<String>, endpoints: Vec<&str>) -> Self {
        Self {
            intent: intent.into(),
            endpoints: endpoints.into_iter().map(|s| s.to_string()).collect(),
            params: HashMap::new(),
        }
    }
}

fn infer_service(user_text: &str) -> Option<String> {
    let t = user_text.to_lowercase();
    for key in [
        "payments",
        "payment",
        "auth-service",
        "auth",
        "orders",
        "order",
        "checkout",
        "billing",
    ] {
        if t.contains(key) {
            let s = match key {
                "payment" => "payments",
                "auth" => "auth-service",
                "order" => "orders",
                _ => key,
            };
            return Some(s.to_string());
        }
    }
    None
}

pub fn heuristic_plan(user_text: &str) -> RouterPlan {
    let t = user_text.to_lowercase();

    let mut plan = if t.contains("ci") || t.contains("pipeline") || t.contains("gitlab") {
        RouterPlan::new("ci_cd_investigation", vec!["/api/gitlab-ci"])
    } else if t.contains("log") || t.contains("container") || t.contains("runtime") {
        RouterPlan::new("logs_fetch", vec!["/api/runtime-logs"])
    } else if t.contains("metric")
        || t.contains("error rate")
        || t.contains("latency")
        || t.contains("observability")
    {
        RouterPlan::new("metrics_check", vec!["/api/observability"])
    } else if t.contains("incident") || t.contains("rollback") {
        RouterPlan::new(
            "incident_review",
            vec!["/api/incident-metrics", "/api/runtime-logs"],
        )
    } else if t.contains("feedback") || (t.contains("user") && t.contains("report")) {
        RouterPlan::new("user_feedback_review", vec!["/api/user-feedback"])
    } else {
        RouterPlan::new(
            "general_ops_question",
            vec!["/api/gitlab-ci", "/api/observability"],
        )
    };

    if let Some(svc) = infer_service(user_text) {
        plan.params.insert("service".into(), svc);
    }
    plan
}

#[derive(Debug, Deserialize)]
struct OaiPlan {
    intent: String,
    endpoints: Vec<String>,
    #[serde(default)]
    params: HashMap<String, String>,
}

pub fn intent_prompt(system_hint: &str, user_text: &str) -> String {
    format!(
        r#"You are a router. Return ONLY a compact JSON with fields: intent (string), endpoints (array of strings), params (object).
Available endpoints:
- "/api/gitlab-ci"              : CI/CD pipelines & jobs
- "/api/runtime-logs"           : container/runtime logs
- "/api/observability"          : SLO, error_rate, p95 latency
- "/api/cloud-mon"              : cloud infra metrics
- "/api/db-perf"                : db query perf & locks
- "/api/mobile-telemetry"       : mobile client telemetry
- "/api/security-auth"          : auth failures, lockouts
- "/api/incident-metrics"       : incidents, MTTR, rollback
- "/api/user-feedback"          : NPS, CSAT, user tickets
- "/api/data-integration-bi"    : BI joins & KPIs
Rules:
1) Pick 1–3 endpoints most relevant.
2) Keep 'params' small (date_from/date_to/tz/service/branch). If user mentions a component/service (e.g., "payments service"), include params.service="<name>".
3) NO prose. Return JSON only.

System hint: {system_hint}

User: {user_text}
"#
    )
}

pub fn parse_or_fallback(json_text: &str, user_text: &str) -> RouterPlan {
    let mut plan = if let Ok(o) = serde_json::from_str::<OaiPlan>(json_text) {
        RouterPlan {
            intent: o.intent,
            endpoints: o.endpoints,
            params: o.params,
        }
    } else {
        heuristic_plan(user_text)
    };
    if plan.endpoints.is_empty() {
        plan = heuristic_plan(user_text);
    }
    if !plan.params.contains_key("service") {
        if let Some(svc) = infer_service(user_text) {
            plan.params.insert("service".into(), svc);
        }
    }
    plan
}

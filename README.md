
# 🚇 SMRT MCP PoC

> **Proof of Concept (PoC)** implementation of the **Model Context Protocol (MCP)** using **Rust**,  
> applied in an **IT Operations** scenario for the **Singapore Mass Rapid Transportation (SMRT)** system.  

⚠️ **Disclaimer**  
This project is for **demonstration & educational purposes only**.  
I am **not affiliated** with the IT Department of SMRT.  

---

## 🧩 What is MCP?

**Model Context Protocol (MCP)** is a standard for connecting AI assistants to external tools, data sources, and APIs.  

Instead of hardcoding application logic or asking users to memorize commands, MCP enables an **AI-driven intent router** that:  

1. Accepts **natural language** queries from users.  
2. Uses an **AI model** (OpenAI Responses API with JSON Schema) to **detect the intent**.  
3. Maps the intent to **one or more API endpoints**.  
4. Fetches and optionally **joins data** from those endpoints.  
5. Returns results to the AI for **human-readable answers** in the chat interface.  

💡 **Example:**  
User: *“Did the last GitLab CI job for the main branch succeed or fail?”*  
- MCP Router detects intent = `ci_status`.  
- Routes to `/api/gitlab-ci`.  
- Fetches dummy JSON with job status + failed tests.  
- AI composes a clear answer for the user.  

👉 With MCP, developers don’t have to build custom logic for each question. Instead, **MCP bridges user intent ↔ system APIs** in a structured, scalable way.

---

## 🔧 Tech Stack

- 🦀 **Backend**: Rust (Axum, SQLx, Reqwest, SSE)  
- ⚡ **Frontend**: Vue 3 + Vite + TypeScript  
- 🗄️ **Database**: MySQL 8  
- 🐳 **Infrastructure**: Docker & Docker Compose  
- 🤖 **AI**: OpenAI GPT (Responses API + JSON Schema)  

---

## 🔄 Sequence Flow

```mermaid
sequenceDiagram
    autonumber
    participant U as User
    participant F as ChatPanel (Vue 3)
    participant B as Backend (Rust/Axum)
    participant R as MCP Router
    participant O as OpenAI (Intent)
    participant E as API Endpoints (dummy)
    participant DB as MySQL

    U->>F: Ask question (natural language)
    F->>B: POST /api/chat (or /api/chat/stream)
    B->>R: Hand off to MCP Router
    R->>O: Intent detection (Responses API + JSON Schema)
    O-->>R: Intent + routing plan (e.g., ci_status → /api/gitlab-ci)
    R->>E: Fetch data from mapped endpoint(s)
    E-->>R: JSON payload(s)
    R->>DB: (Optional) persist/query cached results
    DB-->>R: Data rows (if any)
    R-->>B: Joined + normalized result
    B-->>F: SSE stream (received → llm_start → route_planned → fetch_progress → joined → done)
    F-->>U: Render human-readable answer
````

🌀 SSE Debug Phases:
`received → llm_start → route_planned → fetch_progress → joined → done`

---

## 🏗 Architecture Overview

### Components

* **Frontend (Vue 3 + Vite + TS)** — ChatPanel UI, SSE streaming, status chips (intent/endpoint/phases).
* **Backend (Rust/Axum)** — HTTP API, SSE handler, tracing, error handling.
* **MCP Router (Rust module)** — Prompt Builder → Intent Classifier → Endpoint Planner → Normalizer/Joiner.
* **AI (OpenAI GPT)** — Intent detection with JSON Schema output.
* **Data Sources (Dummy Endpoints)** — `/api/gitlab-ci`, `/api/runtime-logs`, `/api/observability`, `/api/security-auth`, `/api/incident-metrics`, etc.
* **Database (MySQL 8)** — Config & optional cache (`api_results`), future auth (`users`, `sessions`).
* **Infra (Docker & Compose)** — Reproducible local stack.

### High-Level Data Flow

1. User asks in ChatPanel → `POST /api/chat` (or `/api/chat/stream` for SSE).
2. Backend forwards to **MCP Router**.
3. MCP calls **OpenAI** (Responses API + JSON Schema) to get intent & routing plan.
4. MCP fetches from mapped endpoint(s), optionally consults **MySQL**.
5. MCP normalizes/joins → Backend streams via **SSE** → UI renders phases & final answer.

### Architecture Diagram

```mermaid
flowchart TD
  subgraph Client
    U[User] --> F["ChatPanel (Vue 3 + Vite)"]
  end

  subgraph Server["Rust Backend (Axum)"]
    DB[(MySQL 8)]

    F -->|HTTP/SSE| B["API Gateway & SSE Handler"]
    B --> R["MCP Router\n(Prompt Builder • Intent Classifier • Joiner)"]
    R -->|JSON Schema| O["OpenAI Responses API"]

    R --> E1["/api/gitlab-ci/"]
    R --> E2["/api/runtime-logs/"]
    R --> E3["/api/observability/"]
    R --> E4["/api/security-auth/"]
    R --> E5["/api/incident-metrics/"]

    R <-- DB
    B --> DB
  end

  O -.-> R
  E1 -. JSON .-> R
  E2 -. JSON .-> R
  E3 -. JSON .-> R
  E4 -. JSON .-> R
  E5 -. JSON .-> R

  R -->|Normalized Result| B
  B -->|SSE Stream| F
  F --> U

```



## 📊 Suggested Tables (PoC-Friendly)

* `settings` — key/value app configuration.
* `api_results` — simple cache: `endpoint`, `params_hash`, `payload`, `created_at`.
* `users` / `sessions` — for future JWT-based auth.

---


```
## 💬 Example Questions & How MCP Routes Them

Below are 10 natural-language questions and how the **MCP Router** resolves each:
- Detects the **intent** using OpenAI (Responses API + JSON Schema).
- Maps the intent to one or more **API endpoints**.
- Builds **query params** (time range, service, environment, branch).
- **Fetches** and (when needed) **joins** payloads.
- Emits a **normalized result** back to the SSE stream for the UI.

> Response phases on SSE: `received → llm_start → route_planned → fetch_progress → joined → done`

---

### 1) Why did the CI/CD pipeline fail to deploy to staging last night?
- **Intent:** `ci_root_cause`
- **Endpoint(s):** `/api/gitlab-ci`, `/api/deployments`
- **Params:** `env=staging`, `date=yesterday`
- **Notes:** Joins CI job status with deployment attempt; extracts failure summary.
- **Normalized result (shape):**
```json
{
  "intent": "ci_root_cause",
  "ci": { "pipeline_status": "failed", "failed_tests": ["payment_refund_spec", "auth_token_expiry_spec"] },
  "deployments": { "env": "staging", "last_attempt": "failed", "reason": "migration timeout" },
  "answer": "Deployment to staging failed due to a migration timeout; 2 tests failed in CI."
}
````

---

### 2) Can you show me the latest runtime logs for the payments service?

* **Intent:** `logs_fetch`
* **Endpoint(s):** `/api/runtime-logs`
* **Params:** `service=payments`, `limit=200`, `order=desc`
* **Notes:** Streams tail logs; UI truncates or folds lines for readability.
* **Normalized result (shape):**

```json
{ "intent": "logs_fetch", "service": "payments", "lines": ["2025-09-14T... INFO ...", "..."] }
```

---

### 3) How many unresolved tickets are in the observability dashboard right now?

* **Intent:** `observability_ticket_count`
* **Endpoint(s):** `/api/observability`
* **Params:** `metric=unresolved_tickets`, `window=now`
* **Notes:** Returns a single KPI; suitable for a badge or chip in UI.
* **Normalized result (shape):**

```json
{ "intent": "observability_ticket_count", "kpi": { "unresolved_tickets": 17, "as_of": "2025-09-14T10:05:00Z" } }
```

---

### 4) Did the last GitLab CI job for the main branch succeed or fail?

* **Intent:** `ci_status`
* **Endpoint(s):** `/api/gitlab-ci`
* **Params:** `branch=main`, `limit=1`
* **Notes:** Minimal fetch; short, declarative answer.
* **Normalized result (shape):**

```json
{ "intent": "ci_status", "branch": "main", "last_pipeline": { "status": "success", "duration_sec": 612 } }
```

---

### 5) What is the current error rate in the production API gateway?

* **Intent:** `error_rate`
* **Endpoint(s):** `/api/observability`
* **Params:** `service=api-gateway`, `env=prod`, `metric=error_rate`, `window=5m`
* **Notes:** Returns rate plus optional thresholds for coloring.
* **Normalized result (shape):**

```json
{ "intent": "error_rate", "service": "api-gateway", "env": "prod", "error_rate_pct": 0.42, "window": "5m" }
```

---

### 6) Can you compare the deployment duration between staging and production for the last 3 releases?

* **Intent:** `deploy_duration_compare`
* **Endpoint(s):** `/api/deployments`, `/api/releases`
* **Params:** `env=staging,prod`, `limit=3`
* **Notes:** Joins releases→deploys; returns arrays for charting.
* **Normalized result (shape):**

```json
{
  "intent": "deploy_duration_compare",
  "releases": ["v1.12.0","v1.11.3","v1.11.2"],
  "staging_durations_sec": [410, 380, 395],
  "prod_durations_sec": [520, 505, 498]
}
```

---

### 7) Show me the container logs for the auth-service during yesterday’s deployment.

* **Intent:** `logs_during_window`
* **Endpoint(s):** `/api/runtime-logs`, `/api/deployments`
* **Params:** `service=auth-service`, `window=yesterday_deploy_window`
* **Notes:** Determines deploy window from `/api/deployments`, then filters logs to that interval.
* **Normalized result (shape):**

```json
{
  "intent": "logs_during_window",
  "service": "auth-service",
  "window": { "from": "2025-09-13T13:10:00Z", "to": "2025-09-13T13:28:00Z" },
  "lines": ["..."]
}
```

---

### 8) Which microservice caused the rollback in last night’s release?

* **Intent:** `rollback_root_cause`
* **Endpoint(s):** `/api/releases`, `/api/deployments`, `/api/runtime-logs`
* **Params:** `date=yesterday`, `env=prod`
* **Notes:** Multi-endpoint join: find release→detect rollback→scan correlated errors to infer culprit.
* **Normalized result (shape):**

```json
{
  "intent": "rollback_root_cause",
  "release": "v1.12.0",
  "rolled_back": true,
  "culprit_service": "inventory-service",
  "evidence": ["spike 5xx post-deploy", "db deadlock traces"]
}
```

---

### 9) List all failed test cases from the last CI run.

* **Intent:** `ci_failed_tests`
* **Endpoint(s):** `/api/gitlab-ci`
* **Params:** `branch=main` (default), `limit=1`
* **Notes:** Surfaces failed specs for quick triage.
* **Normalized result (shape):**

```json
{ "intent": "ci_failed_tests", "failed_tests": ["checkout_flow_spec", "coupon_apply_spec"] }
```

---

### 10) What is the average response time for the orders API in the past 24 hours?

* **Intent:** `latency_avg`
* **Endpoint(s):** `/api/observability`
* **Params:** `service=orders-api`, `metric=latency_p50`, `window=24h`
* **Notes:** Can return `p50/p95/p99` for richer cards.
* **Normalized result (shape):**

```json
{
  "intent": "latency_avg",
  "service": "orders-api",
  "window": "24h",
  "latency_ms": { "p50": 88, "p95": 210, "p99": 370 }
}
```

---

### How the MCP Server Decides the Endpoint

1. **Intent detection** (LLM): The request body (`text`, optional hints like `env`, `service`) is fed to OpenAI Responses API with a **JSON Schema** that enforces:

   * `intent` (enum of supported intents)
   * `endpoints` (1..N)
   * `params` (validated keys: `env`, `branch`, `date`, `window`, `service`, etc.)
2. **Routing plan**: The MCP Router reads the structured output and builds a plan:

   * Single endpoint (e.g., `ci_status → /api/gitlab-ci`)
   * Multi-endpoint join (e.g., `rollback_root_cause → /api/releases + /api/deployments + /api/runtime-logs`)
3. **Fetch & join**: Executes HTTP calls to dummy endpoints, merges payloads into a **normalized shape**.
4. **Stream to UI**: Streams each phase via **SSE** so the ChatPanel can render progress and final answers.

```

---

## 🚀 Extension Ideas

* 🔗 Plug **Grafana/Prometheus** for real metrics.
* ⚡ Add **rate limits** & **circuit breakers** per endpoint.
* 📝 Persist **audit logs** for prompt, intent, and endpoint calls.
* 🐞 Expose **/internal/debug** for tracing intent & routing decisions.

---

## 👤 Author

**Kukuh Tripamungkas Wicaksono (Kukuh TW)**
💻 Software Architect

* 📧 Email: [kukuhtw@gmail.com](mailto:kukuhtw@gmail.com)
* 📱 WhatsApp: [wa.me/628129893706](https://wa.me/628129893706)
* 🔗 LinkedIn: [linkedin.com/in/kukuhtw](https://www.linkedin.com/in/kukuhtw)
* 🐙 GitHub: [github.com/kukuhtw](https://github.com/kukuhtw)

```

```




## 🔄 Sequence Flow (Mermaid)

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

````

## 🏗 Architecture Overview

### Components
- **Frontend (Vue 3 + Vite + TS)**  
  ChatPanel UI, streams SSE, renders status chips (intent, endpoint, phases).
- **Backend (Rust/Axum)**  
  HTTP API, SSE streaming, error handling, request tracing.
- **MCP Router (Rust module)**  
  - Prompt builder → OpenAI Responses API (JSON Schema)  
  - Intent classifier → endpoint planner (single/multi-endpoint join)  
  - Normalizer/Joiner → consistent response shape for UI
- **AI (OpenAI GPT)**  
  Intent detection & structured output (schema-validated).
- **Data Sources (Dummy Endpoints)**  
  `/api/gitlab-ci`, `/api/runtime-logs`, `/api/observability`, `/api/security-auth`, `/api/incident-metrics`, etc.
- **Database (MySQL 8)**  
  App config, seeds; optional caching table (`api_results`) and auth tables (`users`, `sessions`).
- **Infra (Docker & Compose)**  
  Reproducible builds for backend, frontend, and MySQL.

### High-Level Data Flow
1. User asks a question in ChatPanel → `POST /api/chat` (or `…/stream` for SSE).
2. Backend passes the request to **MCP Router**.
3. Router calls **OpenAI** to classify intent & produce a routing plan (JSON Schema).
4. Router queries one or more **dummy endpoints**, optionally consults **MySQL** (cache/config).
5. Router **joins & normalizes** results → Backend **streams** them to the UI via **SSE**.
6. Frontend renders incremental phases and the final human-readable answer.

### Non-Goals (for this PoC)
- Real integrations (Grafana, Prometheus, GitLab API) — replaced with dummy endpoints.
- Multi-tenant auth (JWT scaffolding listed under “Next Steps”).
- Production-grade observability/security (kept minimal for clarity).

---

### Architecture Diagram (Mermaid)

```mermaid
flowchart TD
    subgraph Client
      U[User] --> F[ChatPanel (Vue 3 + Vite)]
    end

    subgraph Server[Rust Backend (Axum)]
      F -->|HTTP/SSE| B[API Gateway & SSE Handler]
      B --> R[MCP Router<br/>(Prompt Builder • Intent Classifier • Joiner)]
      R -->|JSON Schema| O[OpenAI Responses API]
      R --> E1[/api/gitlab-ci/]
      R --> E2[/api/runtime-logs/]
      R --> E3[/api/observability/]
      R --> E4[/api/security-auth/]
      R --> E5[/api/incident-metrics/]
      R <-- DB[(MySQL 8)]
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
````

### Suggested Tables (optional, PoC-friendly)

* `settings` — key/value app configuration.
* `api_results` — simple cache: `endpoint`, `params_hash`, `payload`, `created_at`.
* `users` / `sessions` — for future JWT-based auth.

### Extension Ideas

* Plug **Grafana/Prometheus** for real metrics.
* Add **rate limits** & **circuit breakers** per endpoint.
* Persist **audit logs** for prompt, intent, and endpoint calls.
* Expose **/internal/debug** for tracing intent & routing decisions.

---


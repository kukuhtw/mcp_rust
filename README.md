````
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
    O-->>R: Intent + routing plan
    R->>E: Fetch data from mapped endpoint(s)
    E-->>R: JSON payload(s)
    R->>DB: (Optional) persist/query cached results
    DB-->>R: Data rows (if any)
    R-->>B: Joined + normalized result
    B-->>F: SSE stream (phases)
    F-->>U: Render answer
````

🌀 SSE Debug Phases:
`received → llm_start → route_planned → fetch_progress → joined → done`

---

## 🏗 Architecture Overview

### Components

* **Frontend (Vue 3 + Vite + TS)** — ChatPanel UI, SSE streaming, status chips.
* **Backend (Rust/Axum)** — HTTP API, SSE handler, tracing, error handling.
* **MCP Router (Rust module)** — Prompt Builder → Intent Classifier → Endpoint Planner → Joiner.
* **AI (OpenAI GPT)** — Intent detection with JSON Schema output.
* **Data Sources (Dummy Endpoints)** — `/api/gitlab-ci`, `/api/runtime-logs`, `/api/observability`, etc.
* **Database (MySQL 8)** — Config & optional cache.
* **Infra (Docker & Compose)** — Reproducible local stack.

### High-Level Data Flow

1. User asks in ChatPanel → `POST /api/chat`.
2. Backend forwards to **MCP Router**.
3. MCP calls **OpenAI** to get intent & routing plan.
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

---

## 📊 Suggested Tables (PoC-Friendly)

* `settings` — key/value app configuration.
* `api_results` — simple cache of API responses.
* `users` / `sessions` — for JWT-based auth (future).

---

## 💬 Example Questions & Endpoints

| #  | Question                                                                                        | Intent                       | Endpoint(s)                                              |
| -- | ----------------------------------------------------------------------------------------------- | ---------------------------- | -------------------------------------------------------- |
| 1  | Why did the CI/CD pipeline fail to deploy to staging last night?                                | `ci_root_cause`              | `/api/gitlab-ci`, `/api/deployments`                     |
| 2  | Can you show me the latest runtime logs for the payments service?                               | `logs_fetch`                 | `/api/runtime-logs`                                      |
| 3  | How many unresolved tickets are in the observability dashboard right now?                       | `observability_ticket_count` | `/api/observability`                                     |
| 4  | Did the last GitLab CI job for the main branch succeed or fail?                                 | `ci_status`                  | `/api/gitlab-ci`                                         |
| 5  | What is the current error rate in the production API gateway?                                   | `error_rate`                 | `/api/observability`                                     |
| 6  | Can you compare the deployment duration between staging and production for the last 3 releases? | `deploy_duration_compare`    | `/api/deployments`, `/api/releases`                      |
| 7  | Show me the container logs for the auth-service during yesterday’s deployment.                  | `logs_during_window`         | `/api/runtime-logs`, `/api/deployments`                  |
| 8  | Which microservice caused the rollback in last night’s release?                                 | `rollback_root_cause`        | `/api/releases`, `/api/deployments`, `/api/runtime-logs` |
| 9  | List all failed test cases from the last CI run.                                                | `ci_failed_tests`            | `/api/gitlab-ci`                                         |
| 10 | What is the average response time for the orders API in the past 24 hours?                      | `latency_avg`                | `/api/observability`                                     |

---

## 🚀 Extension Ideas

* 🔗 Plug **Grafana/Prometheus** for real metrics.
* ⚡ Add **rate limits** & circuit breakers per endpoint.
* 📝 Persist **audit logs** for prompt, intent, and endpoint calls.
* 🐞 Expose **/internal/debug** for tracing intent & routing.

---

## 👤 Author

**Kukuh Tripamungkas Wicaksono (Kukuh TW)**
💻 Software Architect

* 📧 Email: [kukuhtw@gmail.com](mailto:kukuhtw@gmail.com)
* 📱 WhatsApp: [wa.me/628129893706](https://wa.me/628129893706)
* 🔗 LinkedIn: [linkedin.com/in/kukuhtw](https://www.linkedin.com/in/kukuhtw)
* 🐙 GitHub: [github.com/kukuhtw](https://github.com/kukuhtw)

```

---

```

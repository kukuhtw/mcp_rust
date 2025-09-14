Got it 🚀 I’ll rewrite the entire README fully in **English**, keeping the professional style, emojis, and the new **sequence flow diagram**. Here’s the polished version:

```markdown
# 🚇 SMRT MCP PoC

> **Proof of Concept (PoC)** implementation of the **Model Context Protocol (MCP)** using **Rust**,  
> applied in an **IT Operations** scenario for the **Singapore Mass Rapid Transportation (SMRT)** system.  

⚠️ **Disclaimer**  
This project is for **demonstration & educational purposes only**.  
I am **not affiliated** with the IT Department of SMRT.  

---

## 🧩 What is MCP?

**Model Context Protocol (MCP)** is a mechanism that allows an AI Assistant to:  

1. **Understand natural-language questions** from users.  
2. **Detect the intent** (e.g., `logs_fetch`, `ci_status`).  
3. **Map the intent** to the correct **API endpoint(s)**.  
4. **Fetch data** → and let the AI generate a **human-readable answer**.  

💡 **Example Scenario:**  
> User: *“Did the last GitLab CI job for the main branch succeed or fail?”*  
> 🔀 MCP → `/api/gitlab-ci` → returns dummy JSON → AI composes a human-readable response.  

---

## 🔧 Tech Stack

- 🦀 **Backend**: Rust (Axum, SQLx, Reqwest, SSE)  
- ⚡ **Frontend**: Vue 3 + Vite + TypeScript  
- 🗄️ **Database**: MySQL 8  
- 🐳 **Infrastructure**: Docker & Docker Compose  
- 🤖 **AI**: OpenAI GPT (Responses API + JSON Schema)  

---

## 📂 Project Structure

```

smrt-mcp-poc/
├─ backend/       # Rust backend (API + MCP Router)
├─ frontend/      # Vue 3 chat dashboard
├─ data/          # seed/init SQL
├─ migrations/    # schema migrations
├─ docker/        # dockerfiles & compose
└─ README.md

```

---

## ⚡ MCP Endpoint Diagram

This PoC includes **10 dummy endpoints**:

```

/api/runtime-logs     → Synthetic container logs
/api/gitlab-ci        → CI/CD status & failed tests
/api/observability    → Metrics (latency, unresolved tickets, etc.)
/api/security-auth    → Security events (failed logins, errors)
/api/incident-metrics → MTTR, deployment comparisons
/api/test-join        → Join multiple endpoints (dummy test)
/api/settings         → System settings dummy
/api/alerts           → On-call notifications dummy
/api/releases         → Release tracking dummy
/api/deployments      → Deployment metrics dummy

````

📊 **How it works:**  
User Question → MCP Intent Detection → API Endpoint → Fetch Data → AI Response → Chat UI  

---

## 🔄 Sequence Flow

```text
+---------+        +-------------+        +-----------------+        +------------+
|  User   | -----> |  ChatPanel  | -----> |   MCP Router    | -----> |  Endpoint  |
+---------+        +-------------+        +-----------------+        +------------+
     |                   |                         |                        |
     | Ask question       |  POST /api/chat        | Detect intent          |
     |------------------->|----------------------->|----------------------->|
     |                    |                        |   Call API (dummy)     |
     |                    |                        |----------------------->|
     |                    |                        |  Return JSON Response  |
     |                    |<-----------------------|<-----------------------|
     |  AI answer shown   |  Stream via SSE        | Join + Format Result   |
     |<-------------------|<-----------------------|                         |
````

🌀 SSE Debug Phases:
`received → llm_start → route_planned → fetch_progress → joined → done`

---

## 💬 Example Questions

Here are **sample natural-language queries** that can be answered by MCP:

* ❓ *Why did the CI/CD pipeline fail to deploy to staging last night?*
* 📜 *Can you show me the latest runtime logs for the payments service?*
* 📝 *How many unresolved tickets are in the observability dashboard right now?*
* 🔍 *Did the last GitLab CI job for the main branch succeed or fail?*
* ⚠️ *What is the current error rate in the production API gateway?*
* ⏱ *Can you compare the deployment duration between staging and production for the last 3 releases?*
* 📂 *Show me the container logs for the auth-service during yesterday’s deployment.*
* 🛠 *Which microservice caused the rollback in last night’s release?*
* 🧪 *List all failed test cases from the last CI run.*
* 📊 *What is the average response time for the orders API in the past 24 hours?*

---

## ⚙️ Setup & Run

### 1. Clone Repository

```bash
git clone https://github.com/your-org/smrt-mcp-poc.git
cd smrt-mcp-poc
```

### 2. Create `.env`

```env
DATABASE_URL=mysql://smrt:smrtpass@db:3306/smrt_mcp
OPENAI_API_KEY=sk-your-key
OPENAI_MODEL=gpt-4o-mini
SYSTEM_PROMPT=You are an MCP intent router for SMRT IT Department.
RUST_LOG=info
TZ=Asia/Singapore
```

### 3. Run with Docker Compose

```bash
make build
make up
make logs
```

Services:

* Backend → [http://localhost:8080](http://localhost:8080)
* Frontend → [http://localhost:3000](http://localhost:3000)
* MySQL → `localhost:3306` (`smrt/smrtpass`, db: `smrt_mcp`)

---

## ✅ Testing

Health Check:

```bash
curl http://localhost:8080/health
# ok
```

Dummy Join Test:

```bash
curl "http://localhost:8080/api/test-join?date_from=2025-09-14&date_to=2025-09-14&tz=Asia/Singapore"
```

---

## 📌 Next Steps

* 🔗 Integrate with real Observability APIs (Grafana, Prometheus)
* ⚡ Add caching (`api_results`)
* 🔒 Multi-user authentication + JWT
* 📡 Full SSE debug logs with OpenAI streaming

---

## 👤 Author

**Kukuh Tripamungkas Wicaksono (Kukuh TW)**
💻 Software Architect

* 📧 Email: [kukuhtw@gmail.com](mailto:kukuhtw@gmail.com)
* 📱 WhatsApp: [wa.me/628129893706](https://wa.me/628129893706)
* 🔗 LinkedIn: [linkedin.com/in/kukuhtw](https://www.linkedin.com/in/kukuhtw)
* 🐙 GitHub: [github.com/kukuhtw](https://github.com/kukuhtw)

---

```

Do you also want me to add a **Mermaid diagram** version of the sequence flow (so GitHub can render it as an interactive flowchart) alongside the ASCII one?
```

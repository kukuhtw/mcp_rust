
```markdown
# 🚇 SMRT MCP PoC

This repository is a **Proof of Concept (PoC)** implementation of the **Model Context Protocol (MCP)**, built using **Rust**.  
It demonstrates how MCP can be applied in an **IT operations scenario** for the **Singapore Mass Rapid Transportation (SMRT)** system.

⚠️ **Disclaimer**: This project is for **demonstration and educational purposes only**.  
I am **not affiliated** with the IT Department of Singapore Mass Rapid Transportation.  
 

---

## 🧩 What is MCP?

**Model Context Protocol (MCP)** is a mechanism that allows an AI assistant to interpret a user’s natural-language question, determine the correct **intent**, and then fetch relevant data from **external sources** (via APIs).  

In this PoC, MCP routes user questions to **10 different API endpoints** that represent logs and metrics for application development in an IT department setting.  

**Example:**  
- User asks: *“Did the last GitLab CI job for the main branch succeed or fail?”*  
- MCP routes the request to `/api/gitlab-ci`.  
- The endpoint returns dummy JSON with job status and failed test cases.  
- The AI uses that data to generate a human-readable answer.  

This demonstrates how MCP can **bridge user intent with external data sources**.

---

## 🔧 Tech Stack
- **Backend**: Rust (Axum, SQLx, Reqwest, SSE)
- **Frontend**: Vue 3 + Vite + TypeScript
- **Database**: MySQL 8
- **Infra**: Docker & Docker Compose
- **AI**: OpenAI GPT (Responses API, JSON Schema)

---

## 📂 Project Structure
```

smrt-mcp-poc/
├─ backend/         # Rust backend (API + MCP Router)
├─ frontend/        # Vue 3 chat dashboard
├─ data/            # seed/init SQL
├─ migrations/      # schema migrations
├─ docker/          # dockerfiles & compose
└─ README.md

```

---

## ⚡ MCP Endpoint Diagram

Here are the **10 dummy endpoints** implemented in this PoC:  

```

/api/runtime-logs        → Synthetic container logs (payments, auth, etc.)
/api/gitlab-ci           → CI/CD status, failed tests
/api/observability       → Metrics (latency, unresolved tickets, avg response time)
/api/security-auth       → Security events (failed logins, error rates)
/api/incident-metrics    → MTTR, deployment comparisons
/api/test-join           → Multi-endpoint join for testing
/api/settings            → System settings dummy
/api/alerts              → Alerting dummy (on-call notifications)
/api/releases            → Release tracking dummy
/api/deployments         → Deployment metrics dummy

````

📊 **How it works**:  
1. User question is parsed into intent → e.g. `logs_fetch`.  
2. MCP maps intent → correct endpoint(s).  
3. Backend fetches dummy data.  
4. AI composes the response → displayed in the chat UI.  

---

## ⚙️ Setup & Run

### 1. Clone Repo
```bash
git clone https://github.com/your-org/smrt-mcp-poc.git
cd smrt-mcp-poc
````

### 2. Create `.env`

Fill in your OpenAI API key & DB connection string:

```env
DATABASE_URL=mysql://smrt:smrtpass@db:3306/smrt_mcp
OPENAI_API_KEY=sk-your-key
OPENAI_MODEL=gpt-4o-mini
SYSTEM_PROMPT=You are an MCP intent router for SMRT IT Department.
RUST_LOG=info
TZ=Asia/Singapore
```

⚠️ **Never commit `.env`** → already ignored in `.gitignore`.

### 3. Run with Docker Compose

```bash
make build   # build backend & frontend
make up      # start all services
make logs    # view backend logs
```

Services:

* **Backend** → [http://localhost:8080](http://localhost:8080)
* **Frontend** → [http://localhost:3000](http://localhost:3000)
* **MySQL** → localhost:3306 (`smrt/smrtpass`, db: `smrt_mcp`)

---

## 💬 Chat Interface

The **ChatPanel frontend** allows you to ask natural-language questions.

Example prompts:

```
Why did the CI/CD pipeline fail to deploy to staging last night?
Can you show me the latest runtime logs for the payments service?
How many unresolved tickets are in the observability dashboard right now?
Did the last GitLab CI job for the main branch succeed or fail?
What is the current error rate in the production API gateway?
Can you compare the deployment duration between staging and production for the last 3 releases?
Show me the container logs for the auth-service during yesterday’s deployment.
Which microservice caused the rollback in last night’s release?
List all failed test cases from the last CI run.
What is the average response time for the orders API in the past 24 hours?
```

### Request Flow

1. User sends a question → `/api/chat` or `/api/chat/stream` (SSE).
2. MCP performs **intent detection** (via OpenAI).
3. MCP router maps the intent → e.g. `/api/gitlab-ci`, `/api/runtime-logs`.
4. Data is fetched from external endpoints (dummy data in this PoC).
5. Results are joined and streamed back to the UI.

Debug log phases in SSE:
`received → llm_start → route_planned → fetch_progress → joined → done`

---

## 🛠️ Run Without Docker

### Backend

```bash
cd backend
cargo run
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

### Apply DB Migration

```bash
# inside container
make sh-db

# or local
mysql -u smrt -psmrtpass smrt_mcp < migrations/20250914_000001_create_settings.sql
```

---

## ✅ Testing

Health check:

```bash
curl http://localhost:8080/health
# ok
```

Join dummy test:

```bash
curl "http://localhost:8080/api/test-join?date_from=2025-09-14&date_to=2025-09-14&tz=Asia/Singapore"
```

---

## 📌 Next Steps

* Integrate with real Observability APIs (Grafana, Prometheus).
* Add caching for endpoint results (`api_results`).
* Add security and multi-user auth (`users` + JWT).
* Full OpenAI streaming integration with detailed SSE debug logs.

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


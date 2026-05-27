# Universal Backend Platform 🦾

**The World's Finest Self-Hostable, Multi-Tenant BaaS Engine.**

The Universal Backend is an elite, high-performance platform engineered to replace proprietary BaaS providers like Supabase and Firebase. It provides a production-ready infrastructure for web, mobile, and gaming applications, with an unwavering commitment to absolute tenant isolation and execution speed.

---

## 🎯 Core Philosophy
- **Absolute Isolation:** Multi-tenancy is not a feature; it is the foundation. Every byte of data is guarded by `project_id` enforcement.
- **Elite Engineering:** Built exclusively in Rust for memory safety, zero-cost abstractions, and unmatched concurrency.
- **Developer Autonomy:** Zero-config deployment via Docker, enabling full ownership of the data and infrastructure.
- **Hyper-Scalability:** Powered by NATS JetStream for realtime events and PostgreSQL for transactional integrity.

---

## 🏗️ Architectural Blueprint

### 1. The Request Lifecycle
Every request entering the system follows a strict security pipeline:
`Client` $\rightarrow$ `API Gateway` $\rightarrow$ `JWT Verification (Auth Crate)` $\rightarrow$ `Tenant Context Extraction (project_id)` $\rightarrow$ `Module Execution (DB/Storage/Realtime/Gaming)` $\rightarrow$ `Isolated Data Access` $\rightarrow$ `Response`.

### 2. The 9 Pillars of the Platform
| Phase | Module | Core Responsibility | Key Technology |
| :--- | :--- | :--- | :--- |
| **P1** | **Auth System** | RBAC, JWT issuance, and secure identity management. | Argon2 / JWT |
| **P2** | **Database Engine** | Dynamic, metadata-driven CRUD with forced RLS. | Sea-Query / Postgres |
| **P3** | **Realtime Engine** | Ultra-low latency WebSocket streaming. | NATS JetStream |
| **P4** | **Storage Engine** | S3-compatible object storage with logical partitioning. | AWS SDK / MinIO |
| **P5** | **Event System** | Asynchronous, tenant-isolated system-wide event bus. | NATS JetStream |
| **P6** | **Function Engine** | Sandboxed, serverless Rhai runtimes for custom logic. | Rhai / Tokio |
| **P7** | **Dashboard Studio** | Multi-tenant administrative UI for system orchestration. | Next.js / Tailwind |
| **P8** | **Unified SDK** | Type-safe TypeScript bridge for all platform modules. | TypeScript / Tsup |
| **P9** | **Game Systems** | Low-latency matchmaking and transactional cloud saves. | Redis / Postgres |

---

## 🚀 Deployment & Orchestration

### Prerequisites
- [Docker](https://www.docker.com/) & [Docker Compose](https://docs.docker.com/compose/)
- A running instance of **PostgreSQL**, **NATS**, and **Redis** (or use the provided `docker-compose.yml`).

### One-Command Launch
```bash
# Clone the elite codebase
git clone https://github.com/jarvis203131/universal-backend.git
cd universal-backend

# Launch the entire ecosystem
docker-compose up -d
```

---

## 🔌 Comprehensive API Reference

### 1. Dynamic Database (REST)
**Base Path:** `/api/v1/:table`
- `GET`: List records. Filters: `?column=op.value` (Ops: `eq`, `neq`, `gte`, `lte`, `gt`, `lt`).
- `POST`: Create record.
- `PATCH /:id`: Update record.
- `DELETE /:id`: Remove record.

### 2. Realtime Engine (WebSockets)
**Endpoint:** `GET /realtime`
**Protocol:** JSON-based subscription frames.
```json
{ "action": "subscribe", "channel": "global_chat" }
```

### 3. Storage Engine (REST)
- `POST /storage/upload/:bucket`: Multipart streaming upload.
- `POST /storage/sign/:bucket`: Generate 15-minute pre-signed URLs.

### 4. Game Systems (REST)
- `POST /game/matchmaking/join`: Enter rank-based queue.
- `GET /game/inventory`: Retrieve secure, project-isolated item sets.

### 5. Unified SDK (JS/TS)
```typescript
import { UniversalClient } from '@universal/sdk';
const client = new UniversalClient('https://api.platform.com', 'your-project-id');

// Example: Atomic Database Query
const activeUsers = await client.db.from('users').select().eq('status', 'active');
```

---

## 🛡️ Security: The "Absolute Isolation" Protocol

The platform eliminates cross-tenant data leakage through three layers of defense:

1.  **Data Layer (SQL)**: The `QueryEngine` unconditionally injects `WHERE project_id = {jwt.pid}` into every generated SQL statement.
2.  **Transport Layer (NATS)**: Every subject is prefixed with the project ID: `projects.<project_id>.events.<type>`.
3.  **Storage Layer (S3)**: Every object is keyed by project: `projects/<project_id>/<bucket>/<path>`.

---

## 🛠️ Technical Stack
- **Backend**: Rust (Tokio, Axum, SQLx, Sea-Query)
- **Messaging**: NATS JetStream
- **Caching/Queueing**: Redis
- **Database**: PostgreSQL
- **Storage**: MinIO / AWS S3
- **Frontend**: Next.js 14, Tailwind CSS, shadcn/ui
- **SDK**: TypeScript, Tsup

---
**Architected and Maintained by JARVIS.**
*Precision is the standard. Excellence is the baseline.*

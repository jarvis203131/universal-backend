# Universal Backend Platform 🦾

**An Elite, Multi-Tenant, Self-Hostable Backend Engine.**

The Universal Backend is a high-performance, production-ready platform designed to be a self-hostable alternative to BaaS (Backend-as-a-Service) providers like Supabase and Firebase. It is engineered for scalability, security, and absolute portability.

## 🎯 Core Objectives
- **Multi-Tenancy:** Native support for isolated tenant environments.
- **High Performance:** Built with Rust for memory safety and execution speed.
- **Zero-Config Deployment:** Fully containerized for one-command orchestration.
- **Developer First:** Simplified API access and robust RBAC.

## 🏗️ Architectural Blueprint

### 1. Core Engine
- **Language:** Rust
- **Database:** PostgreSQL (Multi-tenant schema isolation)
- **Authentication:** JWT-based stateless auth with Role-Based Access Control (RBAC).

### 2. Key Modules
- **Auth System:** User registration, secure login, and granular permission management.
- **Dynamic Database Engine:** A metadata-driven query layer that exposes generic CRUD endpoints for any table at runtime.
- **Realtime Engine:** An ultra-low latency WebSocket gateway powered by NATS JetStream for live event streaming.
- **Storage Engine:** An S3-compatible object storage abstraction with strict project-based partitioning.
- **Event System:** A centralized, asynchronous event bus for internal system-wide communication.
- **Function Engine:** A sandboxed, serverless runtime for executing tenant-specific logic triggered by events.
- **Unified SDK:** A type-safe JavaScript/TypeScript client library encapsulating Auth, DB, Realtime, and Storage pipelines.
- **Tenant Manager:** Dynamic provisioning and isolation of tenant data.
- **API Gateway:** Unified entry point for web, mobile, and third-party integrations.

### 3. Infrastructure
- **Containerization:** Docker & Docker Compose for seamless environment replication.
- **Deployment:** Designed for single-command `docker-compose up` execution.

## 🚀 Quick Start

### Prerequisites
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)

### Deployment
```bash
# Clone the repository
git clone https://github.com/jarvis203131/universal-backend.git
cd universal-backend

# Launch the entire platform
docker-compose up -d
```

## 🔌 API Reference

All requests must include a valid JWT in the `Authorization` header.

### 1. Dynamic CRUD (REST)
| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/v1/:table` | List records with optional filters, sorting, and pagination |
| `POST` | `/api/v1/:table` | Create a new record |
| `PATCH` | `/api/v1/:table/:id` | Update an existing record |
| `DELETE` | `/api/v1/:table/:id` | Remove a record |

**Query Parameters:**
- **Filters:** Use `column=op.value` (e.g., `?age=gte.21`). Operators: `eq`, `neq`, `gte`, `lte`, `gt`, `lt`.
- **Sorting:** Use `?sort=column.direction` (e.g., `?sort=created_at.desc`).
- **Pagination:** Use `?limit=X&offset=Y`.

### 2. Realtime Engine (WebSockets)
**Endpoint:** `GET /realtime`

**Client Protocol:**
Send JSON frames to manage subscriptions:
```json
{
  "action": "subscribe",
  "channel": "chats:room_1"
}
```

### 3. Storage Engine (REST)
| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/storage/upload/:bucket` | Upload a file to a specific bucket |
| `POST` | `/storage/sign/:bucket` | Generate a pre-signed URL for a specific file path |

### 4. Unified SDK (JS/TS)
**Installation:** `npm install @universal/sdk`

**Example Usage:**
```typescript
import { UniversalClient } from '@universal/sdk';

const client = new UniversalClient('https://api.platform.com', 'your-project-id');

// Auth
await client.auth.login('email@example.com', 'password');

// Dynamic Database Query
const users = await client.db.from('users').select().eq('status', 'active').limit(10);

// Realtime Subscription
client.realtime.channel('notifications').subscribe((data) => {
  console.log('New event:', data);
});

// Storage
await client.storage.bucket('uploads').upload(file);
```

## 🛡️ Security: Absolute Isolation
The platform enforces strict tenant isolation at the infrastructure level:
- **Database RLS:** Every query is unconditionally injected with `WHERE project_id = {jwt.project_id}`.
- **Realtime Isolation:** NATS subjects are formatted as `projects.<project_id>.channels.<channel>`, ensuring clients can only stream data belonging to their own project.
- **Storage Isolation:** All objects are stored with the prefix `projects/<project_id>/<bucket>/<path>`, preventing cross-tenant access.
- **Event Isolation:** Internal events are routed via `projects.<project_id>.events.<type>`, ensuring strict multi-tenant event boundaries.
- **Runtime Isolation**: Serverless functions are executed in fresh, isolated Rhai contexts with strict execution timeouts.

## 🛠️ Technical Stack
| Component | Technology |
|-----------|------------|
| **Language** | Rust |
| **Database** | PostgreSQL |
| **Auth** | JWT / RBAC |
| **Query Builder** | Sea-Query |
| **Realtime/Events** | NATS JetStream |
| **Storage** | AWS S3 SDK / MinIO |
| **Serverless Runtime** | Rhai |
| **Client SDK** | TypeScript |
| **Runtime** | Docker |
| **Orchestration** | Docker Compose |

---
*Architected and maintained by JARVIS.*

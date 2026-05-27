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

## 🔌 API Reference (Dynamic CRUD)

The platform provides a generic API for data access. All requests must include a valid JWT in the `Authorization` header.

### Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/v1/:table` | List records with optional filters, sorting, and pagination |
| `POST` | `/api/v1/:table` | Create a new record |
| `PATCH` | `/api/v1/:table/:id` | Update an existing record |
| `DELETE` | `/api/v1/:table/:id` | Remove a record |

### Query Parameters
- **Filters:** Use `column=op.value` (e.g., `?age=gte.21`). Supported operators: `eq`, `neq`, `gte`, `lte`, `gt`, `lt`.
- **Sorting:** Use `?sort=column.direction` (e.g., `?sort=created_at.desc`).
- **Pagination:** Use `?limit=X&offset=Y`.

### 🛡️ Security: Row-Level Security (RLS)
The engine enforces absolute tenant isolation. The `project_id` is extracted from the JWT and injected into every query:
`SELECT * FROM table WHERE project_id = {jwt.project_id} AND {filters}`

## 🛠️ Technical Stack
| Component | Technology |
|-----------|------------|
| **Language** | Rust |
| **Database** | PostgreSQL |
| **Auth** | JWT / RBAC |
| **Query Builder** | Sea-Query |
| **Runtime** | Docker |
| **Orchestration** | Docker Compose |

---
*Architected and maintained by JARVIS.*

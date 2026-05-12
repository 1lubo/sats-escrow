# SatsEscrow

A Bitcoin escrow service built in Rust, showcasing clean hexagonal architecture and modern async patterns.

## Overview

SatsEscrow enables secure peer-to-peer Bitcoin transactions by holding funds in escrow until both parties fulfill their obligations. The service supports buyer protection, seller guarantees, and dispute resolution through arbitration.

## Architecture

The project follows **Hexagonal Architecture** (Ports & Adapters pattern) with strict separation of concerns:

```
sats-escrow/
├── crates/
│   ├── core/       # Domain logic, escrow state machine, trait definitions (ports)
│   ├── api/        # Axum HTTP routes, extractors, error handling
│   ├── adapters/   # Implementations (mock, MongoDB) for repositories & services
│   └── server/     # Main binary with dependency injection
└── Cargo.toml      # Workspace root
```

### Key Components

| Crate | Purpose |
|-------|---------|
| `sats-escrow-core` | Domain types (`Escrow`, `Dispute`, `User`), state machine, repository traits |
| `sats-escrow-api` | REST endpoints, request/response types, authentication extractors |
| `sats-escrow-adapters` | Mock in-memory implementations + MongoDB repositories |
| `sats-escrow-server` | Wires everything together, starts the HTTP server |

## Current State

### ✅ Completed

- **Escrow State Machine**: `Created → Funded → AwaitingDelivery → Released/Disputed`
- **REST API Endpoints**:
  - `POST /api/v1/escrows` - Create escrow
  - `GET /api/v1/escrows` - List user's escrows
  - `GET /api/v1/escrows/{id}` - Get escrow details
  - `POST /api/v1/escrows/{id}/fund` - Mark as funded
  - `POST /api/v1/escrows/{id}/deliver` - Seller marks delivery
  - `POST /api/v1/escrows/{id}/confirm` - Buyer confirms, releases funds
  - `POST /api/v1/escrows/{id}/dispute` - Open dispute
  - `POST /api/v1/escrows/{id}/cancel` - Cancel (pre-funding only)
- **Mock Adapters**: In-memory implementations for all repositories and services
- **MongoDB Adapters**: Full CRUD implementations for `Escrow`, `Dispute`, `User`
- **Integration Tests**: 8 API tests covering happy paths and error cases
- **Unit Tests**: 9 tests for escrow state machine transitions

### 📊 Test Summary

```
17 tests passing:
  - 9 unit tests (escrow state machine)
  - 8 integration tests (API endpoints)
```

## API Authentication

Currently using a simple bearer token scheme for development:

```
Authorization: Bearer <user-uuid>
```

The token is parsed as a UUID and used as the user ID. Production will require proper JWT/session handling.

## Running Locally

```bash
# Build
cd sats-escrow && cargo build

# Run tests
cargo test

# Start server with mock adapters (no database required)
cargo run

# Start server with MongoDB
MONGODB_URI="mongodb+srv://user:pass@cluster.mongodb.net" cargo run
```

## Deployment to fly.io

### Prerequisites
1. [Install flyctl](https://fly.io/docs/hands-on/install-flyctl/)
2. Create a [MongoDB Atlas](https://www.mongodb.com/cloud/atlas) free cluster
3. Get your MongoDB connection string

### Deploy

```bash
# Login to fly.io
fly auth login

# Launch the app (first time only)
fly launch --no-deploy

# Set MongoDB connection string as a secret
fly secrets set MONGODB_URI="mongodb+srv://user:pass@cluster.mongodb.net/?retryWrites=true&w=majority"

# Deploy
fly deploy

# Check status
fly status

# View logs
fly logs
```

### MongoDB Atlas Setup
1. Create a free M0 cluster at [MongoDB Atlas](https://www.mongodb.com/cloud/atlas)
2. Create a database user with read/write permissions
3. Set network access to "Allow from anywhere" (0.0.0.0/0)
4. Copy the connection string and use it with `fly secrets set`

## Next Steps

### Short Term
- [ ] **Wire MongoDB to server** - Add configuration to switch between mock/MongoDB persistence
- [ ] **Authentication** - Implement proper JWT or session-based auth
- [ ] **Dispute endpoints** - Add resolution and arbitrator assignment routes

### Medium Term
- [ ] **Bitcoin integration** - Connect to Bitcoin node for address generation and payment verification
- [ ] **Webhook system** - Notify parties of escrow state changes
- [ ] **Rate limiting** - Protect API from abuse

### Long Term
- [ ] **Multi-signature escrow** - Use Bitcoin's native multi-sig for trustless escrow
- [ ] **Lightning Network** - Support instant, low-fee payments
- [ ] **Reputation system** - Track user history and trustworthiness

## Tech Stack

- **Language**: Rust 2021 edition
- **Web Framework**: Axum 0.7
- **Database**: MongoDB 2.8
- **Async Runtime**: Tokio
- **Serialization**: Serde

## Git History

| Commit | Description |
|--------|-------------|
| Latest | `chore: add .gitignore` |
| | `test(api): add integration tests for escrow API` |
| | `feat(adapters): add MongoDB repository implementations` |
| | `feat(api): add escrow action endpoints` |
| | `Initial SatsEscrow project structure` |

---

*This project is a portfolio piece demonstrating Rust proficiency and clean architecture patterns.*

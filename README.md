# SatsEscrow

![CI](https://github.com/1lubo/sats-escrow/actions/workflows/ci.yml/badge.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-1.88+-orange.svg)

> A Bitcoin escrow service built in Rust, showcasing hexagonal architecture, domain-driven design, and async patterns.

🤖 **This project was vibecoded** — built collaboratively with AI assistance.

🚀 **Live demo:** [sats-escrow.fly.dev](https://sats-escrow.fly.dev/)

## Architecture

```mermaid
graph LR
    subgraph Core["Core (Domain)"]
        Entities["Escrow · Dispute · User"]
        Ports["Traits / Ports"]
    end

    subgraph API["API Layer"]
        Routes["Axum Routes"]
        Extractors["Auth Extractors"]
        Errors["Error Handling"]
    end

    subgraph Adapters
        Mock["Mock (In-Memory)"]
        Mongo["MongoDB"]
    end

    subgraph Server
        DI["Dependency Injection"]
        Config["Configuration"]
    end

    Routes --> Ports
    Extractors --> Ports
    Mock --> Ports
    Mongo --> Ports
    DI --> Routes
    DI --> Mock
    DI --> Mongo
    Config --> DI
```

## Escrow State Machine

```mermaid
stateDiagram-v2
    [*] --> Created
    Created --> Funded
    Created --> Cancelled
    Funded --> AwaitingDelivery
    AwaitingDelivery --> ReleasedToSeller
    AwaitingDelivery --> Disputed
    AwaitingDelivery --> Cancelled
    Disputed --> ReleasedToBuyer
    Disputed --> ReleasedToSeller
```

## API Endpoints

| Method | Path | Description | Auth |
|--------|------|-------------|------|
| `POST` | `/api/v1/escrows` | Create escrow | ✅ |
| `GET` | `/api/v1/escrows` | List user escrows | ✅ |
| `GET` | `/api/v1/escrows/:id` | Get escrow | — |
| `POST` | `/api/v1/escrows/:id/fund` | Fund escrow | — |
| `POST` | `/api/v1/escrows/:id/deliver` | Mark delivered | ✅ |
| `POST` | `/api/v1/escrows/:id/confirm` | Confirm receipt | ✅ |
| `POST` | `/api/v1/escrows/:id/dispute` | Open dispute | ✅ |
| `POST` | `/api/v1/escrows/:id/cancel` | Cancel escrow | ✅ |
| `GET` | `/api/v1/disputes` | List open disputes | ✅ |
| `GET` | `/api/v1/disputes/:id` | Get dispute | — |
| `POST` | `/api/v1/disputes/:id/vote` | Vote on dispute | ✅ |
| `GET` | `/api/v1/users/me` | Current user | ✅ |
| `GET` | `/api/v1/users/:id/reputation` | User reputation | — |
| `GET` | `/health` | Health check | — |

## Getting Started

### Prerequisites

- **Rust 1.88+** — [install](https://rustup.rs)
- **MongoDB** (optional) — required only for persistent storage
- **Node.js 22+** (optional) — required only for the Svelte frontend

### Quickstart

```bash
# Clone
git clone https://github.com/1lubo/sats-escrow.git
cd sats-escrow

# Build
cargo build

# Run tests
cargo test

# Start server with mock adapters (no database required)
cargo run

# Start server with MongoDB
MONGODB_URI="mongodb+srv://user:pass@cluster.mongodb.net" cargo run
```

The server starts on `http://localhost:3000` by default.

## Project Structure

```
sats-escrow/
└── crates/
    ├── core/       # Domain entities, state machine, repository traits (ports)
    ├── api/        # Axum HTTP routes, request/response types, auth extractors
    ├── adapters/   # Mock in-memory + MongoDB repository implementations
    └── server/     # Binary entry point, dependency injection, configuration
```

| Crate | Role |
|-------|------|
| `sats-escrow-core` | Domain types (`Escrow`, `Dispute`, `User`), state machine, trait definitions |
| `sats-escrow-api` | REST endpoints, error mapping, authentication extractors |
| `sats-escrow-adapters` | Mock in-memory adapters and MongoDB repositories |
| `sats-escrow-server` | Wires adapters into the API and starts the HTTP server |

## Tech Stack

| Technology | Purpose |
|------------|---------|
| Rust | Systems language — safety, performance, concurrency |
| Axum 0.7 | Async web framework built on Tokio and Tower |
| MongoDB 2.0 | Document database for persistent storage |
| Tokio | Async runtime |
| Serde | Serialization / deserialization |
| Svelte 3 | Reactive frontend framework |
| Tailwind CSS | Utility-first CSS |
| Vite | Frontend build tooling |

## License

This project is licensed under the [MIT License](LICENSE).

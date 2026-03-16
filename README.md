# db_test

none

This repository is a **template** to be used with `cargo-generate`. It provides a robust architecture that allows AI coding assistants (like Claude Code, Cursor, or Gemini CLI) to implement business logic with high consistency and clear separation of concerns.

## Quick Start

### 1. Generate Your Project

Install `cargo-generate` if you haven't:
```bash
cargo install cargo-generate
```

Generate a new project from this template:
```bash
cargo generate --git https://github.com/atlas-form/db-center-template.git --name my-new-project
cd my-new-project
```

### 2. Setup Environment

```bash
# Start PostgreSQL (requires Docker)
make postgres

# Edit config/services.toml if needed
cp config/services-example.toml config/services.toml
```

### 3. Let AI Develop Features

Open your AI tool and point it to the protocols:

```text
Please read ai_protocols/TABLE_ADDING_PROTOCOL.md first to understand the project architecture.

Then help me implement a new feature:
<describe your requirements>
```

---

## Architecture

This template follows a strict layered architecture based on the **[db-core-rs](https://github.com/your-username/db-core-rs)** library.

```text
┌──────────────────────────────────────────────────────┐
│                    web-server                        │  Layer 4: HTTP API (Axum)
├──────────────────────────────────────────────────────┤
│                     service                          │  Layer 3: Business Logic (Orchestration)
├──────────────────────────────────────────────────────┤
│                      repo                            │  Layer 2: Data Access (SeaORM Isolation)
├──────────────────────────────────────────────────────┤
│               (External) db-core-rs                  │  Layer 1: Shared Core & Base Traits
├──────────────────────────────────────────────────────┤
│                    migration                         │  Layer 0: Schema (SeaORM Migrations)
└──────────────────────────────────────────────────────┘
```

### Layer Responsibilities

| Layer | Component     | Responsibility                                     | AI Modifiable |
| ----- | ------------- | -------------------------------------------------- | ------------- |
| 4     | `web-server`  | HTTP routes, handlers, OpenAPI, validation         | Yes           |
| 3     | `service`     | Business APIs, orchestrates multiple repos         | Yes           |
| 2     | `repo`        | Single-table services, DTOs, converts Entities     | Yes           |
| 1     | `db-core-rs`  | **External Library**: Base traits, Error, DB Pool  | **Fixed**     |
| 0     | `migration`   | Database schema definitions                        | Yes           |

**Note**: `repo` is the **only** layer allowed to depend on `sea-orm`. This isolates the ORM details from the rest of the application. All business logic in `service` and `web-server` uses pure Rust DTOs.

---

## AI Development Protocols

The project includes detailed guides in `ai_protocols/` that you should always provide to your AI assistant:

| File                                    | Purpose                                |
| --------------------------------------- | -------------------------------------- |
| `ai_protocols/TABLE_ADDING_PROTOCOL.md` | Master execution protocol (Start here) |
| `ai_protocols/MIGRATION_GUIDE.md`       | How to add/modify database tables      |
| `ai_protocols/REPO_GUIDE.md`            | How to implement Data Access Layer     |
| `ai_protocols/SERVICE_GUIDE.md`         | How to implement Business Logic        |
| `ai_protocols/WEB_SERVER_GUIDE.md`      | How to implement HTTP Endpoints        |
| `how_to_use_ai.md`                      | Copy-paste prompt templates            |

---

## Development Commands

```bash
make help              # Show all commands
make postgres          # Start PostgreSQL container
make migrate-up        # Run pending migrations
make migrate-fresh     # Reset DB and run all migrations
make build             # Build all crates
cargo run -p web-server # Start the API server
```

Swagger UI: <http://localhost:19878/swagger-ui>

## Tech Stack

| Component     | Technology          |
| ------------- | ------------------- |
| Runtime       | Tokio               |
| ORM           | SeaORM 2.0          |
| Web Framework | Axum 0.8            |
| OpenAPI       | utoipa + Swagger UI |
| **Core Lib**  | **db-core-rs**      |

## License

MIT or Apache-2.0

# Real-Time WebSocket Broadcast Server

A high-performance, asynchronous WebSocket server built with Rust, Axum, and PostgreSQL featuring JWT-based authentication and secure password hashing.

---

## Tech Stack

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=rust&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-000000?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=for-the-badge&logo=postgresql&logoColor=white)
![JWT](https://img.shields.io/badge/JWT-000000?style=for-the-badge&logo=jsonwebtokens&logoColor=white)
![SQLx](https://img.shields.io/badge/SQLx-000000?style=for-the-badge&logo=rust&logoColor=white)
![Argon2](https://img.shields.io/badge/Argon2-000000?style=for-the-badge&logo=rust&logoColor=white)

---

## Features

- **User Authentication:** Secure signup and login API endpoints.
- **Password Security:** Password hashing powered by Argon2.
- **Protected WebSockets:** Token-based WebSocket connection upgrade via JWT validation.
- **Real-Time Messaging:** Low-latency bi-directional messaging pipeline.
- **Async Runtime:** Built with Tokio and SQLx for efficient non-blocking database queries.

---

## Architecture & Flow

```mermaid
sequenceDiagram
    autonumber
    actor Client
    participant Axum as Axum Router
    participant Auth as Auth Middleware
    participant DB as PostgreSQL DB
    participant WS as WebSocket Handler

    %% Registration
    Client->>Axum: POST /signup {username, password}
    Axum->>DB: Hash Password (Argon2) & Create User
    DB-->>Axum: User Created
    Axum-->>Client: 201 Created

    %% Authentication
    Client->>Axum: POST /login {username, password}
    Axum->>DB: Verify Credentials
    DB-->>Axum: User Verified
    Axum-->>Client: Return JWT Token

    %% WebSocket Connection
    Client->>Axum: Connect ws://localhost:3000/ws?token=JWT
    Axum->>Auth: Validate JWT Token
    alt Valid Token
        Auth-->>WS: Authorize Connection
        Axum-->>Client: 101 Switching Protocols
        loop Real-Time Messaging
            Client->>WS: Message Frame
            WS-->>Client: Process / Echo Message
        end
    else Invalid Token
        Auth-->>Client: 401 Unauthorized
    end

```

---

## Project Structure

```text
ws-broadcast-server/
├── sql/
│   └── migrations/
│       ├── 001_create_users.sql
│       └── 002_create_messages.sql
├── src/
│   ├── handlers/
│   │   ├── mod.rs
│   │   └── ws_handler.rs
│   ├── middleware/
│   │   ├── auth.rs
│   │   └── mod.rs
│   ├── models/
│   │   ├── jwt.rs
│   │   ├── mod.rs
│   │   └── users.rs
│   ├── client.rs
│   ├── database.rs
│   ├── main.rs
│   ├── server.rs
│   └── state.rs
├── .env
├── .env.example
└── .gitignore

```

---

## Getting Started

### 1. Prerequisites

* [Rust & Cargo](https://www.rust-lang.org/)
* [PostgreSQL](https://www.postgresql.org/)
* [wscat](https://github.com/websockets/wscat) *(for testing WebSocket connections)*

### 2. Environment Configuration

Clone the repository and set up environment variables:

```bash
git clone https://github.com/mn409/ws-broadcast-server.git
cd ws-broadcast-server

```

Create a `.env` file matching `.env.example`:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/chat_app
JWT_SECRET=your_super_secret_jwt_key

```

### 3. Database Setup

Run SQL migrations to initialize database schema:

```bash
createdb chat_app
psql -U postgres -d chat_app -f sql/migrations/001_create_users.sql
psql -U postgres -d chat_app -f sql/migrations/002_create_messages.sql

```

### 4. Run the Server

```bash
cargo run -- start

```

The server listens at `http://127.0.0.1:3000`.

---

## Usage Guide

### 1. Register a User

```bash
curl -X POST http://localhost:3000/signup \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","password":"securepassword"}'

```

### 2. Authenticate & Obtain JWT

```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","password":"securepassword"}'

```

*Response:*

```json
{
  "token": "eyJhbGciOiJIUzI1Ni..."
}

```

### 3. Connect to WebSocket Server

Pass your JWT token as a query parameter when upgrading to WebSocket:

```bash
wscat -c "ws://localhost:3000/ws?token=YOUR_JWT_TOKEN"

```

---

## Future Improvements

* **Multi-Client Broadcast:** Implement broadcast channels (`tokio::sync::broadcast`) to send messages across all active clients.
* **Session Tracking:** Manage multi-connection sessions per authenticated user.
* **Chat Rooms & Channels:** Add routing logic for isolated room subscriptions.
* **Persistent Message Storage:** Record incoming message frames into the PostgreSQL database.
* **Token Expiry & Refresh:** Add refresh token workflows for extended sessions.
* **Structured Logging:** Implement observability using `tracing` and `tracing-subscriber`.
* **Containerization & Deployment:** Add `Dockerfile` configuration for automated deployment on Fly.io / Railway.

---

## Repository

[https://github.com/mn409/ws-broadcast-server]

# Real-Time WebSocket Broadcast Server

A concurrent, asynchronous WebSocket server built in Rust supporting multi-client messaging with persistent storage in PostgreSQL.

---

## Architecture

The system follows a layered backend design:

Client -> WebSocket -> Axum Router -> Handler -> Shared State -> PostgreSQL

* **Axum** handles HTTP routing and WebSocket protocol upgrades.
* **Tokio** provides the asynchronous runtime for concurrent connection handling.
* **Shared State** (`Arc<Mutex<Vec<Sender>>>`) tracks and manages connected client sockets.
* **PostgreSQL** persists users and incoming messages.

---

## Features

* Real-time WebSocket communication
* Multi-client concurrent connection management
* System-wide broadcast messaging
* Persistent message and user storage with PostgreSQL
* Automatic user registration via initial message handshake
* Automatic cleanup for disconnected clients

---

## Tech Stack

* **Language:** Rust
* **Framework:** Axum
* **Runtime:** Tokio
* **Protocol:** WebSockets
* **Database:** PostgreSQL
* **Database Driver:** SQLx

---

## Project Structure

```text
.
├── src/
│   ├── main.rs        # Application entry point
│   ├── server.rs      # Axum server configuration and routing
│   ├── handlers.rs    # WebSocket lifecycle and frame handling
│   ├── state.rs       # Shared application state definition
│   └── database.rs    # PostgreSQL connection pooling
└── sql/
    └── schema.sql     # Database initialization schema

```

---

## Getting Started

### 1. Prerequisites

Ensure you have Rust (cargo) and PostgreSQL installed on your system.

### 2. Environment Configuration

Create a `.env` file in the root directory:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/chat_app

```

### 3. Database Initialization

Create the database and apply the schema:

```bash
createdb chat_app
psql -U postgres -d chat_app -f sql/schema.sql

```

---

## Usage

### Start the Server

```bash
cargo run -- start

```

The server listens for WebSocket connections at:
`ws://127.0.0.1:3000/ws`

### Connect a Client

In a separate terminal window, launch a client instance:

```bash
cargo run -- connect

```

*You can open multiple terminal sessions to simulate concurrent users.*

### Interaction Protocol

1. **First Input:** Sets your active username and registers the user.
2. **Subsequent Inputs:** Sends messages that are persisted to PostgreSQL and broadcast to all connected clients.

---

## How It Works

1. **Connection Setup:** A client initiates an HTTP request to `/ws`, which Axum upgrades to a WebSocket connection.
2. **Socket Splitting:** The socket splits into separate `Sender` and `Receiver` halves. The `Sender` half is stored in the thread-safe application state.
3. **Message Handling:**
* **Handshake:** The first received payload registers the user in the database.
* **Broadcasting:** Subsequent payloads are saved to PostgreSQL and sent to all active `Sender` sockets.


4. **Cleanup:** When a client drops the connection, its `Sender` handle is cleared from the shared state.

---

## Roadmap

* Implement JWT-based authentication
* Replace `Mutex<Vec<Sender>>` with Tokio broadcast channels (`tokio::sync::broadcast`) for scalable message distribution
* Add endpoint for fetching historical messages
* Support multi-room and channel routing
* Containerize application using Docker

---

## Repository

[https://github.com/mn409/ws-broadcast-server](https://github.com/mn409/ws-broadcast-server)
# WebSocket Broadcast Server (Rust)

A simple CLI-based WebSocket broadcast server built in Rust using Axum.

The server allows multiple clients to connect and exchange messages in real time. Any message sent by one client is broadcast to all connected clients.

Project (this repo):
https://github.com/mn409/ws-broadcast-server

Original project idea:
https://roadmap.sh/projects/broadcast-server

## Features

- WebSocket server using Axum
- Multiple clients can connect at the same time
- Messages are broadcast to all connected clients
- CLI interface to start the server or connect as a client
- Handles client disconnects

## Usage

### Start the server

```
cargo run start
```

The server will run at:

```
ws://127.0.0.1:3000/ws
```

---

### Connect as a client

Open another terminal and run:

```
cargo run connect
```

You can open multiple terminals to simulate multiple clients.

---

### Send messages

Type a message in any client terminal and press enter.

The message will be sent to the server and broadcast to all connected clients.

## How it works

- The server keeps a list of connected clients
- When a client sends a message, it is forwarded to every other client
- If sending to a client fails, that client is removed

## Tech Stack

- Rust
- Tokio
- Axum
- WebSockets

## Project Structure

```
src/
  main.rs
  server.rs
  client.rs
  handlers.rs
  state.rs
```

## Notes

This is a basic implementation focused on understanding WebSockets and async handling in Rust. It can be extended with features like usernames, chat rooms, or message history.

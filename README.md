# The Konqueror

A collaborative Command and Control (C2) framework written in Rust.

Rewrite of the [original Go version](https://github.com/ctrlc03/the-konqueror) with a redesigned architecture, pluggable transports, per-implant encryption, and malleable C2 profiles.

## Architecture

```
Operator (CLI)  <── REST + WS ──>  Server  <── WS ──>  Listener  <── HTTP/2 ──>  Implant
```

- **Server** — Central teamserver. REST API for operators, WebSocket relay for listeners. Manages state, auth, tasking, and events.
- **Client** — Operator CLI for interacting with the server.
- **Listener** — Proxy between implants and the server. Accepts implant connections over configurable transports (HTTP, DNS, TCP, SMB) and relays to the server over WebSocket.
- **Implant** — Agent running on target. Supports beacon (poll) and session (interactive) modes.

## Workspace

```
crates/
  common/          Shared types, error handling, protocol definitions, crypto
  storage/         Storage trait + implementations (in-memory, SQLite)
  server/          Teamserver (axum REST API + WebSocket)
  client/          Operator CLI
  listener-http/   HTTP listener
  implant/         Implant agent
```

## Building

```sh
cargo build --workspace
```

### Build the server

```sh
cargo run -p konqueror-server -- --address 127.0.0.1 --port 9002
```

## Key Design Decisions

- **No gRPC** — REST for CRUD, WebSocket for push. Simpler, fewer dependencies, better traffic blending.
- **Pluggable storage** — `Storage` trait with async methods. Swap SQLite for Postgres or in-memory for tests.
- **Per-implant crypto** — X25519 key exchange on first check-in, then AES-256-GCM + HMAC-SHA256 with per-implant session keys.
- **Malleable profiles** — TOML-based C2 profiles for traffic shaping (transform pipelines, custom headers, URI rotation).
- **Feature-gated implant** — Cargo feature flags select which commands compile into the implant binary. Keeps it small.

## Status

Work in progress. Phase 1 (foundation) under active development.

## License

MIT

# Redis Clone in Rust

A Redis-inspired in-memory key-value database built completely from scratch in Rust.

The purpose of this project is not to replace Redis, but to deeply understand the engineering behind high-performance network servers, concurrent data structures, database internals, and asynchronous systems programming.

This project will progressively evolve from a simple TCP server into a feature-rich Redis-compatible server supporting persistence, expiration, Pub/Sub, transactions, and replication.

---

## Project Goals

- Learn advanced Rust through real-world systems programming
- Understand asynchronous networking with Tokio
- Implement the Redis Serialization Protocol (RESP)
- Design a concurrent in-memory database
- Build a production-style command execution pipeline
- Explore persistence mechanisms (AOF and snapshots)
- Learn distributed systems concepts through replication
- Produce a clean, modular, and maintainable Rust codebase

---

## Planned Features

### Networking

- TCP server
- Concurrent client handling
- Async I/O with Tokio
- Graceful connection handling

---

### Redis Protocol (RESP)

- RESP2 parser
- RESP serializer
- Streaming parser
- Binary-safe strings

---

### Commands

- PING
- ECHO
- GET
- SET
- DEL
- EXISTS
- EXPIRE
- TTL
- INCR
- DECR
- MGET
- MSET

---

### Storage Engine

- In-memory key-value store
- Thread-safe shared state
- Key expiration
- Atomic operations
- Efficient lookups

---

### Persistence

- Append Only File (AOF)
- AOF replay
- Snapshotting (RDB-inspired)
- Crash recovery

---

### Pub/Sub

- SUBSCRIBE
- UNSUBSCRIBE
- PUBLISH
- Multiple channels

---

### Transactions

- MULTI
- EXEC
- DISCARD
- Command queue

---

### Replication

- Primary/Replica synchronization
- Command propagation
- Initial database sync

---

### Future Ideas

- Redis Streams
- Sorted Sets
- Lua scripting
- ACL Authentication
- Cluster mode
- Metrics endpoint
- Benchmark suite
- Custom CLI client

---

## Architecture

```
                 Client
                    │
                    │ TCP
                    ▼
        Tokio TCP Listener
                    │
                    ▼
         Connection Handler
                    │
                    ▼
            RESP Parser
                    │
                    ▼
           Command Parser
                    │
                    ▼
          Command Executor
                    │
          ┌─────────┴─────────┐
          ▼                   ▼
      Database           Background Tasks
          │
          ▼
      Persistence
```

---

## Project Structure

```
src/

├── server/
├── protocol/
├── command/
├── storage/
├── persistence/
├── replication/
├── pubsub/
├── config/
├── error.rs
├── lib.rs
└── main.rs
```

---

## Learning Objectives

This project focuses on mastering advanced Rust concepts including:

- Ownership
- Borrowing
- Lifetimes
- Traits
- Generics
- Associated Types
- Smart Pointers
- Interior Mutability
- Async/Await
- Tokio Runtime
- Concurrency
- Channels
- Arc
- Mutex
- RwLock
- Error Handling
- Iterators
- Closures
- Macros
- Serialization
- Networking
- File I/O
- State Machines
- Protocol Design

---

## Tech Stack

- Rust
- Tokio
- bytes
- thiserror
- anyhow
- tracing
- tracing-subscriber

Additional crates may be introduced as the project evolves.

---

## Development Roadmap

### Phase 1

- [ ] Project setup
- [ ] TCP server
- [ ] Multiple client support

### Phase 2

- [ ] RESP parser
- [ ] RESP serializer

### Phase 3

- [ ] Command parser
- [ ] Command dispatcher

### Phase 4

- [ ] Database engine
- [ ] Shared state
- [ ] Basic commands

### Phase 5

- [ ] Expiration system
- [ ] Background cleanup task

### Phase 6

- [ ] Append Only File (AOF)

### Phase 7

- [ ] Pub/Sub

### Phase 8

- [ ] Transactions

### Phase 9

- [ ] Replication

### Phase 10

- [ ] Performance optimizations

---

## Why Build This?

Redis appears simple from the outside:

```
SET user "Alice"
GET user
```

Internally, however, it combines several complex systems:

- Network server
- Binary protocol parser
- Concurrent storage engine
- Memory management
- Persistence layer
- Replication engine
- Background workers
- Event-driven architecture

Building these components from scratch provides practical experience with systems programming concepts that apply far beyond Redis.

---

## Inspiration

This project draws inspiration from:

- Redis
- Tokio
- mini-redis
- "Build Your Own Redis" educational resources

The implementation is written independently for learning purposes and is not affiliated with the Redis project.

---

## Contributing

This repository is primarily an educational project. Suggestions, issues, and pull requests are welcome if they improve code quality, architecture, documentation, or performance.

---

## License

MIT License

Markdown# ⚡ Anza Lab —  Thread-Safe Financial Ledger API

**Anza Lab** is an asynchronous REST API developed in **Rust** designed to simulate a financial ledger. Built according to strict **Clean Architecture** principles, the project ensures complete decoupling between business rules, domain contracts, and persistence mechanisms.

The system focuses on **memory safety, native thread-safety, and zero-cost abstractions**, serving as a solid foundation for off-chain financial engines, settlement systems, and Zero-Knowledge (ZK) cryptography infrastructure.

---


## 🏗️ Architecture

The project is structured into isolated layers to guarantee testability, maintainability, and seamless infrastructure swapping.

```
src/
│
├── 🧩 domain/                  # Domain Layer — Entities & Abstract Contracts
│   ├── models.rs               # Ledger entities (e.g., Transferencia)
│   └── traits.rs               # ContaRepository trait (Send + Sync)
│
├── ⚙️  application/             # Application Layer — Use Cases
│   └── services.rs             # TransferenciaService (Injected via Generics)
│
├── 🛠️  infrastructure/          # Infrastructure Layer — Persistence & Concurrency
│   └── database/
│       └── repos.rs            # InMemoryContaRepository (Arc<RwLock<>>)
│
└── 🌐 main.rs                  # Presentation Layer — Axum HTTP Server & Handlers
```

### Layer Responsibilities

| Layer | Responsibility | Key File |
|---|---|---|
| 🧩 **Domain** | Core business rules, entities and repository contracts | `traits.rs` |
| ⚙️ **Application** | Orchestrates use cases via generic dependency injection | `services.rs` |
| 🛠️ **Infrastructure** | Concrete persistence implementation (in-memory, future: PostgreSQL) | `repos.rs` |
| 🌐 **Presentation** | HTTP routing, request deserialization and response handling | `main.rs` |
### Layer Breakdown

* **`domain`**: Contains pure data structures and contracts (`traits`). The `ContaRepository` trait enforces `Send + Sync`, enabling safe data sharing across multiple threads.
* **`application`**: Encapsulates business logic inside `TransferenciaService`. Uses **Generics** (`<R: ContaRepository>`) for zero-cost dependency injection.
* **`infrastructure`**: Implements in-memory persistence using `Arc<RwLock<HashMap<String, u64>>>`. Ensures concurrent access without data races or underflows.
* **`main`**: HTTP router built with **Axum** and **Tokio**, managing atomic shared state (`State`) and exposing REST endpoints.

---

## 🛠️ Tech Stack

* **Language:** Rust (Edition 2021)
* **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
* **Async Runtime:** [Tokio](https://tokio.rs/)
* **Serialization/Deserialization:** [Serde](https://serde.rs/) / `serde_json`
* **Concurrency Primitives:** `std::sync::Arc`, `std::sync::RwLock`

---

## 🔒 Concurrency & Thread-Safety Model

To handle concurrent HTTP requests without unnecessary lock contention, the in-memory state is guarded by an `Arc<RwLock<HashMap>>`:

1. **Concurrent Shared Reads (`GET /saldo/:conta`):**
   The `buscar_saldo` method acquires a shared read lock (`RwLock::read`), allowing hundreds of concurrent requests to read account balances in parallel without blocking each other.
2. **Atomic Exclusive Writes (`POST /transferir`):**
   The `atualizar_saldo` method acquires an exclusive write lock (`RwLock::write`), ensuring that only one thread mutates account balances at any given moment, preventing race conditions and double-spending.

---

## 🚀 Getting Started

### Prerequisites
* [Rust & Cargo](https://www.rust-lang.org/tools/install) installed.

### Running the Application

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/your-username/anza_lab.git](https://github.com/your-username/anza_lab.git)
   cd anza_lab

## ✅ Verify & Run

```bash
# Check the project
cargo check

# Run the server
cargo run
```

The server will start at `http://localhost:3000`.

---

## 📡 API Endpoints

### 1. Process Transfer

| Field   | Value                      |
|---------|----------------------------|
| URL     | `/transferir`              |
| Method  | `POST`                     |
| Headers | `Content-Type: application/json` |

**Payload:**
```json
{
  "de": "conta1",
  "para": "conta2",
  "valor": 1000
}
```

**cURL:**
```bash
curl -X POST http://localhost:3000/transferir \
  -H "Content-Type: application/json" \
  -d '{"de": "conta1", "para": "conta2", "valor": 1000}'
```

**Response:**


---

### 2. Get Account Balance

| Field  | Value           |
|--------|-----------------|
| URL    | `/saldo/:conta` |
| Method | `GET`           |

**cURL:**
```bash
curl http://localhost:3000/saldo/conta1
```

**Response:**


---

## 🧪 Initial State Data

The in-memory repository (`InMemoryContaRepository`) is seeded with default accounts upon startup:

| Account | Initial Balance |
|---------|----------------|
| conta1  | 5,000          |
| conta2  | 5,000          |

---

## 🗺️ Roadmap & Future Enhancements

- [ ] **Semantic HTTP Errors** — Implement `AppError` enum with Axum's `IntoResponse` to return proper HTTP status codes (`400 Bad Request`, `404 Not Found`)
- [ ] **Structured JSON Responses** — Replace raw string responses with JSON DTOs (`SaldoResponse`)
- [ ] **Observability & Telemetry** — Integrate `tracing` and `tracing-subscriber` crates for structured request logging
- [ ] **Relational Persistence** — Implement a PostgreSQL repository layer using `sqlx`
- [ ] **Concurrent Stress Testing** — Build an automated integration test suite using `tokio::spawn` for high-concurrency load testing

---

## 📜 License

Developed as part of research and continuous learning in systems engineering, high performance, and Clean Architecture in Rust.
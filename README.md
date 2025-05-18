# Concurrency vs. Parallelism in Rust (Tokio)

A micro‑demo that runs **two CPU‑bound async tasks** twice:

1. **Single worker thread**: tasks run concurrently on one core
2. **Two worker threads**: tasks run in parallel on two cores

---

## Build & run

```bash
cargo run --release
```

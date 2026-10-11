# Phase 3: advanced, on demand

Only the topics your phase 2 notes flagged. One file per topic in `src/bin/`.

```
cargo run -p phase3 --bin scratch
```

| Topic | Exercise idea |
|---|---|
| lifetimes | struct holding a `&str`, fn returning one of two borrowed inputs |
| `Rc`/`RefCell` | tree or graph with shared nodes |
| `Arc`/`Mutex`, threads | counter incremented from 10 threads |
| channels | worker pool with `mpsc` |
| async (`tokio`) | fetch 3 URLs concurrently. `cargo add tokio --features full -p phase3` |
| error crates | rework ex04 with `thiserror` + `anyhow` |
| traits, advanced | associated types, `impl Iterator` returns, operator overloading |
| macros | `macro_rules!` that builds a `HashMap` literal |

## Rules
Same as phase 1. Also: write the Java version in your head first, then work out what Rust does differently and why.

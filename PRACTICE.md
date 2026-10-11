# Rust in practice (vs the JVM)

No VM. Your binary **is** the process. Nothing sits between your code and the OS.

## Crashes

| | JVM | Rust |
|---|---|---|
| uncaught error, main thread | stack trace, JVM exits once non-daemon threads finish | panic message, exit code 101 |
| uncaught error, other thread | thread dies, app keeps running | same (`join()` → `Err`) |
| stack overflow | `StackOverflowError`, catchable | process **aborts**. Not catchable |
| out of memory | `OutOfMemoryError` | allocation failure → abort |
| native crash | `hs_err_pid.log` | segfault. Only possible via `unsafe`/FFI |
| crash dump | heap dump, `hs_err` | OS core dump, if enabled |

- Nothing restarts you. Use systemd/launchd/k8s/a supervisor.
- Log panics yourself: `std::panic::set_hook(...)`, or let `tracing` + a panic hook do it.
- `std::process::exit(n)` skips `Drop`. Return from `main` instead (`fn main() -> Result<(), E>` or `ExitCode`).
- SIGTERM/Ctrl-C kill immediately, no shutdown hooks. Handle them with `tokio::signal` or the `ctrlc` crate.

## Memory
- No GC, no `-Xmx`, no GC pauses, no tuning flags. Memory is freed at scope end, deterministically.
- RAII: files/sockets/locks close in `Drop`. No `try-with-resources` needed. That's just how it works.
- Leaks are still possible: `Rc` cycles, `Box::leak`, ever-growing maps.
- Stack vs heap matters: big arrays on the stack + deep recursion = abort.

## Performance
- Starts instantly. No warmup, no JIT. Peak speed from the first request.
- **Debug builds are 10–100× slower.** Benchmark and ship with `--release`, always.
- Profiling: `cargo flamegraph`, Instruments (macOS), `perf` (Linux). No JFR/VisualVM.
- Debugging: lldb/gdb, or the RustRover debugger. No attach agents, no hot code reload.

## Distribution
- One native binary per OS/CPU target. No "write once, run anywhere". Build per target (CI matrix or `cross`).
- No runtime to install. Docker: `FROM gcr.io/distroless/cc` or `scratch` (with musl) → images of a few MB.
- Linux: links against glibc. Build on the oldest distro you support, or target `x86_64-unknown-linux-musl` for a fully static binary.
- macOS: code signing + notarization if you distribute outside the App Store.

## No runtime reflection
- No classpath scanning, no runtime annotations, no proxies. That means no Spring, no Hibernate, no Mockito-style magic.
- Their equivalents work at compile time: `#[derive(Serialize)]` (serde), `sqlx::query!` (SQL checked at compile time).
- DI = pass dependencies as struct fields/params. Shared state = `Arc<AppState>`.
- Mocking = put a trait at the seam, implement a fake (or `mockall`).
- No runtime plugin loading (dylibs exist but are rare and painful).

## Ecosystem defaults you must pick

| Need | Java default | Rust (de facto) |
|---|---|---|
| async runtime | built in (threads, virtual threads) | `tokio` |
| HTTP server | Spring Web | `axum` |
| HTTP client | `HttpClient` | `reqwest` |
| JSON | Jackson | `serde` + `serde_json` |
| logging | SLF4J + Logback | `tracing` + `tracing-subscriber` |
| CLI args | picocli | `clap` |
| errors | exceptions | `anyhow` (apps), `thiserror` (libs) |
| DB | JDBC/JPA | `sqlx`, `diesel` |
| config | system properties | env vars, `config`/`figment` crates |

Logging prints **nothing** until you initialize a subscriber in `main`.

## Async gotchas
- Futures are lazy: nothing runs until `.await` or spawn.
- A blocking call (file IO, `std::thread::sleep`, heavy CPU) inside async stalls a whole runtime worker. Use `tokio::task::spawn_blocking`.
- `async` spreads up the call chain ("function coloring"). No virtual threads to hide it.

## Dependencies
- No jar hell at runtime. Everything is resolved and linked at compile time.
- Two versions of the same crate can coexist in one binary. Their types are **not** interchangeable (confusing errors).
- `cargo audit` ≈ OWASP dependency-check. `cargo deny` for licenses/bans.

## Interop
- C ABI directly via `extern "C"`. No JNI pain. `bindgen` generates bindings from C headers.
- Calling Rust from Java: JNI (`jni` crate) or the Panama FFM API.

## Build & CI
- Compiles slower than Java. Cache `~/.cargo` + `target/` in CI (`Swatinem/rust-cache` on GitHub Actions).
- `cargo clippy -- -D warnings` + `cargo fmt --check` + `cargo test` = the standard CI gate.

# Libraries

What each dependency does, with the Java equivalent. Based only on the `Cargo.toml` files.

## Crate map (inferred from names + deps)

| crate | type | role |
|---|---|---|
| `sf-proto` | lib | shared protocol/message types (serde), socket IO |
| `sf-hold` | lib + bin | pty/session holder. The only crate allowed `unsafe` |
| `sf-driver` | lib | spawns and drives agent subprocesses |
| `sfd` | lib + bin | the daemon: SQLite, signals, keychain, notifications, HTTP |
| `slop` | bin | CLI that talks to `sfd` |
| `sf-app` | lib + bin | desktop GUI (egui), tray icon, embedded terminal |
| `sf-theme`, `sf-widgets` | lib | UI design tokens and reusable widgets |
| `parity` (`sf-parity`) | tests | screenshot comparisons of v2 screens against v1 |
| `tests` (`sf-chaos`) | tests | chaos tests: kill -9 subprocesses, check recovery |
| `uat` (`sf-uat`) | tests | end-to-end acceptance tests |

## Core

| crate | what | Java |
|---|---|---|
| `serde` | (de)serialization framework. `#[derive(Serialize, Deserialize)]` generates the code at compile time | Jackson annotations |
| `serde_json` | JSON format for serde | Jackson `ObjectMapper` |
| `anyhow` | one catch-all error type + `.context("...")`. For apps | `throws Exception` + wrapping |
| `thiserror` | `#[derive(Error)]` for your own error enums. For libs | custom exception hierarchy |
| `chrono` | dates/times/timezones | `java.time` |
| `uuid` | UUIDs. `v4` = random, `v5` = name-based hash | `java.util.UUID` |
| `blake3` | very fast cryptographic hash | `MessageDigest` (SHA-256) |
| `clap` | CLI arg parsing via `#[derive(Parser)]` on a struct | picocli |

## Async & logging

| crate | what | Java |
|---|---|---|
| `tokio` | async runtime: scheduler, timers, sockets, processes, signals, channels. Features enable only what you use (`net`, `time`, `process`, …) | Netty + `ExecutorService` + NIO |
| `tracing` | structured logging + spans (`info!`, `#[instrument]`) | SLF4J + MDC |
| `tracing-subscriber` | the backend that prints logs. `env-filter` = `RUST_LOG=debug` | Logback + logback.xml |

## OS / system

| crate | what | Java |
|---|---|---|
| `libc` | raw C bindings to the OS (unsafe) | JNI/Panama |
| `nix` | safe Rust wrappers over Unix APIs: processes, signals, ptys, terminals | none (JNA libs) |
| `rusqlite` | SQLite. `bundled` compiles SQLite's C source into your binary, so there's no system lib | sqlite-jdbc |
| `keyring` | OS secret store (macOS Keychain) | `java-keyring` |
| `notify-rust` | desktop notifications | `SystemTray.displayMessage` |
| `ureq` | small **blocking** HTTP client, TLS via `rustls` (pure Rust, no OpenSSL) | `HttpClient` (sync) |
| `tray-icon` | menu bar / system tray icon | `java.awt.SystemTray` |

## GUI

| crate | what | Java |
|---|---|---|
| `egui` | **immediate-mode** GUI: you redraw the whole UI every frame from your state. No widget objects, no listeners | none. Opposite of Swing/JavaFX |
| `eframe` | the window + render loop that hosts egui | `JFrame` |
| `alacritty_terminal` | terminal emulator core from Alacritty (parses VT escape codes, keeps the grid) | JediTerm |
| `egui_kittest` | test harness: simulate clicks, render frames to PNG snapshots | AssertJ-Swing + screenshot tests |
| `image` | image encode/decode (PNG here, for snapshots) | `ImageIO` |

## Test-only

| crate | what | Java |
|---|---|---|
| `assert_cmd` | run your binary, assert on exit code/stdout | ProcessBuilder in a test |
| `tempfile` | temp files/dirs, deleted on `Drop` | JUnit `@TempDir` |

## Cargo patterns worth noticing
- **`"=1.0.104"`**: exact pin. The default `"1.0.104"` means `^1.0.104`, i.e. any compatible 1.x. Pinning ≈ Maven's fixed versions, no ranges.
- **`features = [...]`**: compile only the parts you use. Smaller binary, faster build.
- **`default-features = false`**: turn off the crate's defaults, then opt in (`keyring`, `ureq`, `image`).
- **`{ path = "../sf-proto" }`**: local crate dependency ≈ a sibling Maven module.
- **`[workspace.package]` + `edition.workspace = true`**: inherited settings ≈ a parent POM.
- **`[workspace.lints.rust] unsafe_code = "forbid"`**: build fails on any `unsafe`. `forbid` can't be overridden, so `sf-hold` and `sfd` skip the workspace lints and set their own: `allow`, and `deny`. A `deny` can still be lifted per module with `#![allow(unsafe_code)]`.
- **`[profile.dev] debug = 0`**: no debug info, to keep `target/` small (see [Tooling](TOOLING.md)).
- **`[profile.dev.package.X] opt-level = 1/3`**: optimize only hot dependencies (SQLite, egui, PNG) in dev builds; your own code stays unoptimized and debuggable.
- **`[[bin]]`, `[[example]]`, `[[test]]`**: extra build targets. `harness = false` means the test file has its own `fn main()` instead of using `#[test]`.
- **`[dev-dependencies]`**: test scope. **`publish = false`**: never push to crates.io.

---

# Spring Boot stack → Rust

There's no Spring. Rust backends are assembled from separate crates, and you wire them together yourself in `main`. The de facto stack is **tokio + axum + serde + sqlx + tracing**.

| Layer | Spring Boot | Rust | Notes |
|---|---|---|---|
| framework / DI | Spring Boot, `@Autowired` | **none** | No IoC container. Build your deps in `main` and pass them as `State<Arc<AppState>>` |
| web / REST | Spring Web MVC, WebFlux | `axum` (alt: `actix-web`) | Handlers are plain async fns. Routes are built in code, not annotations |
| server / event loop | Tomcat, Netty | `hyper` + `tokio` | axum runs on hyper. tokio ≈ Netty's event loop |
| middleware, filters, AOP | Servlet filters, `@Aspect` | `tower` / `tower-http` layers | CORS, timeouts, compression, request tracing. No AOP: cross-cutting concerns go in layers |
| JSON | Jackson | `serde` + `serde_json` | Compile-time, no reflection |
| validation | Hibernate Validator, `@Valid` | `validator`, `garde` | `#[validate(email)]` on struct fields |
| ORM | Spring Data JPA + Hibernate | `sqlx` (SQL), `SeaORM` (closest to JPA), `diesel` | **No Hibernate equivalent**: no lazy loading, dirty checking or session cache. Most teams write SQL with sqlx, checked at compile time against the real DB |
| repositories | `findByNameAndAge()` | **none** | No derived queries. Write the SQL or use SeaORM's query builder |
| transactions | `@Transactional` | `let tx = pool.begin().await?; …; tx.commit().await?` | Explicit. Dropping `tx` without commit rolls back |
| connection pool | HikariCP | built into `sqlx` (`PgPool`), `deadpool` | |
| migrations | Flyway, Liquibase | `sqlx migrate`, `refinery` | Plain `.sql` files, like Flyway |
| config | `application.yml`, `@ConfigurationProperties` | `config` / `figment` + a serde struct, `dotenvy` | Deserialize config into a typed struct at startup |
| logging | SLF4J + Logback, `@Slf4j` | `tracing` + `tracing-subscriber` | |
| metrics, tracing | Actuator, Micrometer | `metrics`, `opentelemetry` + `tracing-opentelemetry` | No `/actuator` for free. Add a health route yourself |
| security | Spring Security | **none** | Assemble from pieces: `jsonwebtoken`, `argon2`, `axum-login`, tower middleware |
| HTTP client | RestTemplate, WebClient, Feign | `reqwest` | |
| OpenAPI | springdoc | `utoipa` | Derive macros on handlers + types |
| scheduling | `@Scheduled` | `tokio::time::interval`, `tokio-cron-scheduler` | |
| async | `@Async`, executors | `tokio::spawn` | |
| caching | `@Cacheable`, Caffeine | `moka` | moka ≈ Caffeine |
| messaging | Spring Kafka, Spring AMQP | `rdkafka`, `lapin` | |
| DTO mapping | MapStruct | `impl From<Entity> for Dto` | Plain code |
| testing | JUnit, Mockito, MockMvc, Testcontainers | built-in `#[test]`, `mockall`, `axum-test`, `testcontainers` | |

## Lombok: mostly unnecessary, and no getters/setters

| Lombok | Rust |
|---|---|
| `@ToString` | `#[derive(Debug)]` |
| `@EqualsAndHashCode` | `#[derive(PartialEq, Eq, Hash)]` |
| `@Data` / `@Value` | `#[derive(Debug, Clone, PartialEq)]` + fields |
| `@NoArgsConstructor` | `#[derive(Default)]` |
| `@AllArgsConstructor`, `@RequiredArgsConstructor` | struct literal: `User { name, age }` |
| `@Builder` | `User { age: 3, ..Default::default() }`, or the `bon` crate |
| `@Getter` / `@Setter` | **no equivalent, and usually not wanted** |
| `@Slf4j` | not needed: `tracing::info!` works anywhere |

**Why no getters/setters:** in Java, getters protect fields from outside mutation. In Rust the borrow checker already does that. Someone holding a `&User` can read `user.name` but can't change it, even when the field is `pub`. So the convention is:
- plain data (DTOs, config, rows): `pub` fields, no accessors
- types with invariants: private fields + methods, written by hand (`fn age(&self) -> u32`)

The `getset` crate generates accessors, but it's unidiomatic.

## Minimal REST endpoint (axum)
```rust
use axum::{Json, Router, extract::{Path, State}, routing::get};
use serde::Serialize;
use std::sync::Arc;

struct AppState { greeting: String }          // your "beans"

#[derive(Serialize)]
struct User { id: u32, name: String }

async fn get_user(State(st): State<Arc<AppState>>, Path(id): Path<u32>) -> Json<User> {
    Json(User { id, name: format!("{} #{id}", st.greeting) })   // Json<T> = @ResponseBody
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = Arc::new(AppState { greeting: "user".into() });   // manual DI
    let app = Router::new()
        .route("/users/{id}", get(get_user))   // @GetMapping("/users/{id}")
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```
`cargo add axum tokio --features tokio/full serde --features serde/derive anyhow`

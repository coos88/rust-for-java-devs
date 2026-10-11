# Libraries in slopforge v2

What each dependency in `slopforge2/v2` does, with the Java equivalent. Based only on the `Cargo.toml` files.

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

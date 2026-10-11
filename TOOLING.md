# Tooling

## The pieces

| Rust | Java |
|---|---|
| `rustup` | SDKMAN. Installs/switches toolchains (`rustup update`, `rustup default stable`). This repo needs Rust ≥ 1.88 |
| `rustc` | `javac`, but outputs a native binary. You rarely call it directly |
| `cargo` | Maven + `java` launcher + dependency manager, in one |
| `Cargo.toml` | `pom.xml` |
| `Cargo.lock` | resolved exact versions (Maven has no real equivalent). Commit it for apps |
| crates.io | Maven Central |
| crate | artifact / jar |
| workspace | multi-module Maven project (this repo is one) |
| `target/` | `target/` (yes, same name) |
| edition (`2024`) | language version / `--release`. Crates of different editions mix fine |
| `clippy` | SpotBugs/Sonar, built in |
| `rustfmt` | google-java-format, built in |
| rust-analyzer | the IDE language server (in RustRover/VS Code) |

## Cargo vs mvn

| cargo | mvn |
|---|---|
| `cargo new app` / `--lib` | archetype:generate |
| `cargo check` | compile, minus codegen. **Fastest feedback, use this constantly** |
| `cargo build` | compile (debug profile, `target/debug/`) |
| `cargo build --release` | optimized build (`target/release/`). Slow to compile, fast to run |
| `cargo run` | build + run. `cargo run -- args` passes args |
| `cargo test` | test |
| `cargo add serde` | add dependency (edits Cargo.toml) |
| `cargo update` | refresh lockfile within semver ranges |
| `cargo tree` | dependency:tree |
| `cargo clippy` | lint |
| `cargo fmt` | format |
| `cargo doc --open` | javadoc, for your code + all deps |
| `cargo clean` | clean (frees the GBs) |
| `cargo install foo` | install a CLI tool to `~/.cargo/bin` |
| `-p phase1 --bin ex01_basics` | `-pl module`, pick package + binary |

## Cargo.toml
```toml
[package]
name = "app"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }   # "1" means ^1.0.0 (semver caret by default)
tokio = { version = "1", features = ["full"] }

[dev-dependencies]          # scope test
pretty_assertions = "1"

[profile.release]           # build settings per profile
lto = true
```
**Features** = compile-time optional parts of a crate (≈ Maven optional deps + profiles, but resolved at compile time).

## How compilation works
- **Unit = crate**, not a file. Your crate plus every dependency is compiled **from source** on your machine.
- Pipeline: parse → macro expansion → type check + **borrow check** → LLVM → machine code → link into one binary. All crates are linked in statically; only the system libc is linked dynamically.
- No runtime and no VM. The binary is everything, built for one OS/CPU target (`--target x86_64-unknown-linux-gnu` cross-compiles).
- **Generics are monomorphized**: `Vec<i32>` and `Vec<String>` each get their own machine code. Fast at runtime, slow to compile.
- Incremental compilation caches the work for your crate. Deps compile once per profile/feature set.
- Errors include a code (`E0382`). `rustc --explain E0382` gives the full explanation.

## Why builds eat GBs
1. **Every dep compiled from source, per project.** Maven shares one `~/.m2` of prebuilt jars. Cargo shares only the *source* (`~/.cargo/registry`). Each project's `target/` holds its own compiled copy.
2. **Debug info.** Debug builds embed full DWARF symbols for every crate, often more than half the size.
3. **Incremental cache.** `target/debug/incremental` stores intermediate compiler state so rebuilds stay fast. It's large and grows.
4. **Monomorphization.** Generic code gets duplicated for each concrete type.
5. **Multiple variants.** debug, release, test, `check` metadata, and each feature combination are stored separately.
6. **Never pruned.** Old artifacts stay until you `cargo clean`.

### Fixes

| | |
|---|---|
| `cargo clean` | nuke it. Rebuild costs minutes, not data |
| workspace | members share one `target/` (that's why this repo is a workspace) |
| `[profile.dev] debug = "line-tables-only"` | much smaller, stack traces still work. Use `debug = false` to go further |
| `CARGO_TARGET_DIR=~/.cargo-target` | one shared target dir for all projects. Deps get reused where versions/features match |
| `cargo install cargo-sweep` → `cargo sweep --time 30` | delete artifacts older than 30 days |
| `sccache` | shared compilation cache across projects (≈ a build cache) |

## This repo
```
Cargo.toml          workspace root
phase1-core/        cargo run -p phase1 --bin ex01_basics
phase2-your-app/    notes while reading your real app
phase3-advanced/    cargo run -p phase3 --bin scratch
```

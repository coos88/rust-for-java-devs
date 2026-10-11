# Phase 2: read a real app

Apply phase 1 to an existing Rust codebase. Write notes, not code.

The app here is slopforge v2. [Libraries](../LIBRARIES.md) already covers step 3, so start at step 2, then step 4.

## Steps
1. Pick the app. Write its path or URL at the top of `01-overview.md`.
2. `cargo build`, `cargo test`, `cargo doc --open`. Get it running.
3. `01-overview.md`: what `Cargo.toml` pulls in, and what each dependency is for.
4. `02-entrypoint.md`: walk through `main.rs` (or `lib.rs`) line by line.
5. One note per module after that: `03-<module>.md`, …

## Per note
- What it does, in one sentence.
- Rust concepts that show up (link to the cheat sheet section).
- Anything you couldn't explain → **Questions** list at the bottom.

## Watch for
`Arc`/`Mutex`, lifetimes (`'a`), `async`/`.await`, `impl Trait`, `dyn`, macros, `unsafe`. Note where each one shows up. Those become phase 3.

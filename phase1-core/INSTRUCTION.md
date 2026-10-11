# Phase 1: core language

Six exercises in `src/bin/`. Each file lists its tasks at the top.

```
cargo run -p phase1 --bin ex01_basics
```

| # | Topic | Cheat sheet |
|---|---|---|
| 01 | variables, types, control flow, functions | Variables & types, Control flow, Functions |
| 02 | ownership, moves, borrowing (**take your time**) | Ownership |
| 03 | structs, impl, enums, match | Structs & impl, Enums |
| 04 | Option, Result, `?`, panics | Option & Result |
| 05 | traits, generics, `dyn` | Traits, Generics |
| 06 | collections, iterators, closures | Collections, Iterators |

## Rules
- Replace each `todo!()`. Don't change function signatures unless the task says so.
- Read every compiler error in full. It usually tells you the fix.
- `cargo check` for fast feedback. `cargo clippy` once it compiles.
- No `.clone()` to silence the borrow checker unless the task allows it.
- Done = it compiles, it runs, there are no warnings, and you can explain why.

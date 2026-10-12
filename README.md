# Rust for Java devs

Compact Rust reference and exercises, mapped from Java.

- [Cheat sheet](CHEATSHEET.md): the whole language
- [Tooling](TOOLING.md): cargo vs mvn, the compiler, why `target/` gets huge
- [In practice](PRACTICE.md): crashes, memory, deployment, ecosystem. Life without a JVM
- [Libraries](LIBRARIES.md): every crate the app uses, plus the Spring Boot stack mapped to Rust
- [Examples](EXAMPLES.md): 38 everyday recipes + 15 compile errors everyone hits

## Exercises
- [Phase 1: core language](phase1-core/INSTRUCTION.md)
- [Phase 2: read a real app](phase2-your-app/INSTRUCTION.md)
- [Phase 3: advanced, on demand](phase3-advanced/INSTRUCTION.md)

```
git clone https://github.com/coos88/rust-for-java-devs
cd rust-for-java-devs
cargo run -p phase1 --bin ex01_basics
```

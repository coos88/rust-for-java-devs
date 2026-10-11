# Everyday Rust

Small recipes for things you'll do all the time. Crates in brackets go in via `cargo add <crate>`.
Some examples use `struct User { name: String, age: u32 }`.

## Program structure

### 1. A main that can fail [anyhow]
```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let text = std::fs::read_to_string("app.json").context("reading app.json")?;
    println!("{} bytes", text.len());
    Ok(())
}
```
Returning `Result` from `main` lets you use `?` everywhere. On error, Rust prints the whole context chain ("reading app.json: No such file…") and exits with code 1.

### 2. Load a config file into a typed struct [serde (features: derive), serde_json]
```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    port: u16,
    #[serde(default = "default_host")] host: String,   // missing → default_host()
    #[serde(default)] debug: bool,                      // missing → false
}
fn default_host() -> String { "127.0.0.1".into() }

fn load_config(path: &str) -> anyhow::Result<Config> {
    Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
}
```
Like `@ConfigurationProperties`: the file is checked against the struct once, at startup. A wrong type or a missing required field is an error right there, not a null later.

### 3. Read an env var with a default
```rust
fn env_or<T: std::str::FromStr>(key: &str, default: T) -> T {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn demo() {
    let port: u16 = env_or("PORT", 8080);
    let verbose: bool = env_or("VERBOSE", false);
}
```
One generic helper works for any type that can be parsed from a string. A missing variable and an unparseable value both fall back to the default.

## Working with data

### 4. Count word frequencies
```rust
use std::collections::HashMap;

fn word_counts(text: &str) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w.to_lowercase()).or_insert(0) += 1;
    }
    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));   // most frequent first
    sorted
}
```
`entry().or_insert(0)` returns a mutable reference to the value, so `*… += 1` increments it in place. HashMaps aren't sorted, so you collect into a Vec to sort.

### 5. Group items by a key
```rust
use std::collections::HashMap;

fn by_age(users: &[User]) -> HashMap<u32, Vec<&User>> {
    let mut groups: HashMap<u32, Vec<&User>> = HashMap::new();
    for u in users {
        groups.entry(u.age).or_default().push(u);
    }
    groups
}
```
The equivalent of `Collectors.groupingBy`. `or_default()` creates an empty Vec the first time a key shows up. The groups hold `&User`, so nothing is copied.

### 6. Sort by multiple keys
```rust
fn sort_users(users: &mut [User]) {
    // age descending, then name ascending
    users.sort_by(|a, b| b.age.cmp(&a.age).then_with(|| a.name.cmp(&b.name)));
}
```
`then_with` is `Comparator.thenComparing`. Swapping `a` and `b` in the first comparison reverses the order.

### 7. Remove duplicates but keep the order
```rust
use std::collections::HashSet;

fn unique(items: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    items.into_iter().filter(|x| seen.insert(x.clone())).collect()
}
```
`HashSet::insert` returns `false` when the value was already there, so the filter keeps only first occurrences. `Vec::dedup` only removes *adjacent* duplicates.

### 8. Parse many values: all-or-nothing vs skip the bad ones
```rust
fn parse_all(input: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    input.iter().map(|s| s.parse::<i32>()).collect()   // stops at the first error
}

fn parse_valid(input: &[&str]) -> Vec<i32> {
    input.iter().filter_map(|s| s.parse().ok()).collect()   // silently drops failures
}
```
Collecting an iterator of `Result`s into a `Result<Vec<_>, _>` gives `Ok(all values)` or the first `Err`. `filter_map` with `.ok()` turns each failure into `None`, and `None`s are skipped.

### 9. Parse a string into your own enum
```rust
use std::str::FromStr;

enum Level { Low, High }

impl FromStr for Level {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Level::Low),
            "high" => Ok(Level::High),
            other => Err(format!("unknown level: {other}")),
        }
    }
}

fn demo() -> Result<(), String> {
    let lvl: Level = "HIGH".parse()?;   // .parse() works because of FromStr
    Ok(())
}
```
Implementing `FromStr` hooks your type into `.parse()`, the same method you use for numbers. It's the counterpart of a static `valueOf(String)`.

### 10. Model a state machine with an enum
```rust
enum Order {
    New,
    Paid { amount: u32 },
    Shipped { tracking: String },
}

impl Order {
    fn advance(self, input: &str) -> Order {       // takes self: the old state is consumed
        match self {
            Order::New => Order::Paid { amount: input.parse().unwrap_or(0) },
            Order::Paid { .. } => Order::Shipped { tracking: input.to_string() },
            done @ Order::Shipped { .. } => done,
        }
    }
}
```
Each state carries only the data valid in that state, so there's no nullable `tracking` field on an unpaid order. Because `advance` consumes `self`, nobody can keep using the old state.

### 11. Newtypes for type safety
```rust
struct UserId(u64);
struct OrderId(u64);

fn cancel(order: OrderId, by: UserId) { }

fn demo() {
    cancel(OrderId(7), UserId(1));
    // cancel(UserId(1), OrderId(7));   ❌ compile error: arguments swapped
}
```
Wrapping a primitive in a one-field struct makes mixing up IDs a compile error. At runtime it's just a `u64`: zero cost.

### 12. Many optional fields: Default + struct update
```rust
#[derive(Debug, Default)]
struct Request { url: String, timeout_secs: u64, retries: u32, verbose: bool }

fn demo() {
    let r = Request { url: "https://x.dev".into(), retries: 3, ..Default::default() };
}
```
Set the fields you care about, and `..Default::default()` fills in the rest. This replaces most uses of a Lombok `@Builder`.

### 13. Make your type iterable
```rust
struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 { return None; }
        self.0 -= 1;
        Some(self.0 + 1)
    }
}

fn demo() {
    let v: Vec<u32> = Countdown(3).collect();             // [3, 2, 1]
    let evens: u32 = Countdown(10).filter(|n| n % 2 == 0).sum();
}
```
Implement one method, `next`, and you get every iterator adapter for free (`map`, `filter`, `sum`, `collect`, `for` loops…).

## Errors

### 14. Your own error type [thiserror]
```rust
#[derive(Debug, thiserror::Error)]
enum StoreError {
    #[error("item {0} not found")]
    NotFound(u32),
    #[error("storage failed")]
    Io(#[from] std::io::Error),
}

fn load(id: u32) -> Result<String, StoreError> {
    let data = std::fs::read_to_string("store.txt")?;   // io::Error → StoreError::Io via #[from]
    data.lines().nth(id as usize).map(String::from).ok_or(StoreError::NotFound(id))
}
```
Callers can `match` on the variants, much like catching specific exception subclasses. `#[from]` lets `?` convert the underlying error automatically.

### 15. Turn a missing value into an error [anyhow]
```rust
use anyhow::Context;
use std::collections::HashMap;

fn find_email(users: &HashMap<u32, String>, id: u32) -> anyhow::Result<&String> {
    let email = users.get(&id).with_context(|| format!("no user {id}"))?;
    Ok(email)
}
```
anyhow's `.context()` works on `Option` too: `None` becomes an error with your message. Use `with_context` with a closure so the message is only built when it's actually needed.

### 16. Early return with let-else
```rust
fn port_of(addr: &str) -> Option<u16> {
    let Some((_, port)) = addr.rsplit_once(':') else {
        return None;
    };
    port.parse().ok()
}
```
A guard clause: unwrap the happy path into a variable or bail out. It keeps the main logic unindented, instead of nesting it inside `if let`.

## Files & processes

### 17. Read a large file line by line
```rust
use std::io::{BufRead, BufReader};

fn count_errors(path: &str) -> std::io::Result<usize> {
    let file = std::fs::File::open(path)?;
    let mut n = 0;
    for line in BufReader::new(file).lines() {
        if line?.contains("ERROR") { n += 1; }
    }
    Ok(n)
}
```
`BufReader` reads in chunks, so memory stays flat however big the file is. Each line is a `Result` because reading can fail halfway.

### 18. Write a file efficiently
```rust
use std::io::{BufWriter, Write};

fn write_csv(rows: &[(String, u32)]) -> std::io::Result<()> {
    let mut out = BufWriter::new(std::fs::File::create("report.csv")?);
    writeln!(out, "name,count")?;
    for (name, n) in rows {
        writeln!(out, "{name},{n}")?;
    }
    out.flush()   // also happens on drop, but then errors are silently lost
}
```
Without `BufWriter`, every `writeln!` is a separate system call. The file closes automatically when `out` goes out of scope.

### 19. Walk a directory recursively
```rust
use std::path::{Path, PathBuf};

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() { walk(&path, out)?; } else { out.push(path); }
    }
    Ok(())
}
```
The standard library has no `Files.walk`, but recursion over `read_dir` is short. For filters and symlink handling, use the `walkdir` crate.

### 20. Process stdin like a Unix tool
```rust
fn main() -> std::io::Result<()> {
    for line in std::io::stdin().lines() {
        println!("{}", line?.to_uppercase());
    }
    Ok(())
}
```
Run it as `cat file.txt | cargo run`. Startup is instant, which makes Rust well suited to small CLI tools.

### 21. Run a command and use its output
```rust
use std::process::Command;

fn current_branch() -> anyhow::Result<String> {
    let out = Command::new("git").args(["branch", "--show-current"]).output()?;
    if !out.status.success() {
        anyhow::bail!("git failed: {}", String::from_utf8_lossy(&out.stderr));
    }
    Ok(String::from_utf8(out.stdout)?.trim().to_string())
}
```
`output()` waits and captures stdout/stderr as bytes. `bail!` is anyhow's shorthand for `return Err(anyhow!(...))`.

### 22. Stream a command's output live
```rust
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn follow() -> std::io::Result<()> {
    let mut child = Command::new("ping").args(["-c", "3", "localhost"]).stdout(Stdio::piped()).spawn()?;
    let stdout = child.stdout.take().expect("piped above");
    for line in BufReader::new(stdout).lines() {
        println!("> {}", line?);
    }
    child.wait()?;
    Ok(())
}
```
`spawn()` starts the process without waiting, and reading the pipe line by line shows output as it arrives. Always `wait()` at the end, or the finished process lingers as a zombie.

## CLI

### 23. Subcommands [clap (features: derive)]
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Add { name: String },                     // app add <name>
    List { #[arg(long)] all: bool },          // app list --all
}

fn main() {
    match Cli::parse().cmd {
        Cmd::Add { name } => println!("adding {name}"),
        Cmd::List { all } => println!("listing, all={all}"),
    }
}
```
The enum *is* the CLI definition, like `git add` / `git list`. clap generates `--help`, error messages and parsing, and the `match` forces you to handle every subcommand.

## JSON & HTTP

### 24. Enums as tagged JSON [serde, serde_json]
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Event {
    Login { user: String },
    Logout { user: String, reason: Option<String> },
}

fn demo() -> serde_json::Result<()> {
    let e: Event = serde_json::from_str(r#"{"type":"login","user":"coos"}"#)?;
    Ok(())
}
```
Jackson's `@JsonTypeInfo` + `@JsonSubTypes` in two lines. `Option` fields accept both `null` and a missing key.

### 25. Call a JSON API [ureq (features: json), serde]
```rust
#[derive(serde::Deserialize)]
struct Repo { name: String, stargazers_count: u32 }

fn stars(owner: &str, repo: &str) -> anyhow::Result<u32> {
    let url = format!("https://api.github.com/repos/{owner}/{repo}");
    let r: Repo = ureq::get(&url).header("User-Agent", "demo").call()?.body_mut().read_json()?;
    Ok(r.stargazers_count)
}
```
Blocking and simple, fine for CLIs and scripts. In async code (axum, tokio) use `reqwest` instead. Fields missing from the struct are ignored, so declare only what you need.

## Concurrency

### 26. Split work across threads
```rust
fn parallel_sum(data: &[u64]) -> u64 {
    let chunk = data.len().div_ceil(4).max(1);
    std::thread::scope(|s| {
        let handles: Vec<_> = data.chunks(chunk).map(|c| s.spawn(move || c.iter().sum::<u64>())).collect();
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })
}
```
Scoped threads may borrow `data` directly, because the scope guarantees they finish before the function returns. No `Arc`, no copying. For bigger jobs, the `rayon` crate does this with `.par_iter()`.

### 27. Collect results from workers over a channel
```rust
use std::sync::mpsc;
use std::thread;

fn run_workers() {
    let (tx, rx) = mpsc::channel();
    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || tx.send(format!("worker {id} done")).unwrap());
    }
    drop(tx);                    // drop the original sender, or the loop below never ends
    for msg in rx {
        println!("{msg}");
    }
}
```
The receiver loop ends once every sender is gone. That's why the original `tx` is dropped explicitly: forgetting it is a classic hang.

### 28. A cache shared between threads
```rust
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Default)]
struct Cache { map: RwLock<HashMap<String, String>> }

impl Cache {
    fn get(&self, k: &str) -> Option<String> { self.map.read().unwrap().get(k).cloned() }
    fn put(&self, k: String, v: String) { self.map.write().unwrap().insert(k, v); }
}
```
Share it as `Arc<Cache>`. Note the methods take `&self`, not `&mut self`: the lock provides the mutation (many readers *or* one writer, like `ReentrantReadWriteLock`).

### 29. Run async tasks concurrently [tokio (features: full)]
```rust
use tokio::task::JoinSet;

async fn fetch(id: u32) -> u32 { id * 10 }

async fn fetch_all() -> anyhow::Result<Vec<u32>> {
    let mut set = JoinSet::new();
    for id in 1..=5 {
        set.spawn(fetch(id));
    }
    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        results.push(res?);      // res is Err if the task panicked
    }
    Ok(results)                  // in completion order, not spawn order
}
```
Like submitting to an `ExecutorService` and draining a `CompletionService`. Dropping the `JoinSet` cancels whatever is still running.

### 30. Give up after a timeout [tokio]
```rust
use std::time::Duration;

async fn slow() -> u32 { tokio::time::sleep(Duration::from_secs(5)).await; 1 }

async fn with_deadline() -> anyhow::Result<u32> {
    let v = tokio::time::timeout(Duration::from_secs(2), slow()).await?;   // Err(Elapsed) after 2s
    Ok(v)
}
```
Wraps any future. When the time runs out, the inner future is simply dropped, which cancels it. No interrupt flags needed.

### 31. Retry with exponential backoff [tokio, anyhow]
```rust
use std::time::Duration;

async fn flaky() -> anyhow::Result<String> { anyhow::bail!("service down") }

async fn with_retry() -> anyhow::Result<String> {
    let mut delay = Duration::from_millis(100);
    for attempt in 1..=5 {
        match flaky().await {
            Ok(v) => return Ok(v),
            Err(e) if attempt < 5 => {
                eprintln!("attempt {attempt} failed: {e}, retrying in {delay:?}");
                tokio::time::sleep(delay).await;
                delay *= 2;
            }
            Err(e) => return Err(e.context("gave up after 5 attempts")),
        }
    }
    unreachable!()
}
```
The match guard (`if attempt < 5`) separates "retry" from "give up" without nested ifs. The `backon` crate packages this up if you need it in many places.

### 32. Run something every N seconds [tokio]
```rust
use std::time::Duration;

async fn heartbeat() {
    let mut tick = tokio::time::interval(Duration::from_secs(30));
    loop {
        tick.tick().await;       // first tick fires immediately
        println!("still alive");
    }
}
```
`@Scheduled(fixedRate = 30000)`. Start it with `tokio::spawn(heartbeat())` so it runs alongside the rest of the app.

### 33. Graceful shutdown on Ctrl-C [tokio]
```rust
async fn serve() { loop { tokio::time::sleep(std::time::Duration::from_secs(1)).await; } }

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = serve() => {}
        _ = tokio::signal::ctrl_c() => println!("shutting down"),
    }
    // cleanup here: flush, close connections. Drop runs for everything in scope
}
```
`select!` waits for whichever finishes first and cancels the other. Without this, Ctrl-C kills the process immediately: no shutdown hook, no `Drop`.

## Time

### 34. Measure how long something takes
```rust
use std::time::Instant;

fn timed() {
    let start = Instant::now();
    let total: u64 = (0..1_000_000).sum();
    println!("sum={total} took {:?}", start.elapsed());   // "took 1.2ms"
}
```
`Instant` is monotonic, like `System.nanoTime()`. `{:?}` formats the duration readably. Measure with `--release`, since debug builds are far slower.

### 35. Parse dates and do date math [chrono]
```rust
use chrono::{NaiveDate, TimeDelta};

fn due_in() -> anyhow::Result<i64> {
    let start = NaiveDate::parse_from_str("2026-10-11", "%Y-%m-%d")?;
    let due = start + TimeDelta::days(30);
    let today = chrono::Local::now().date_naive();
    Ok((due - today).num_days())
}
```
`NaiveDate` = `LocalDate` (no timezone). `DateTime<Utc>` = `Instant`/`ZonedDateTime`. Subtracting two dates gives a `TimeDelta`.

## Logging & tests

### 36. Set up logging [tracing, tracing-subscriber (features: env-filter)]
```rust
fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    tracing::info!(user = "coos", attempts = 3, "login ok");
    tracing::debug!("only shown with RUST_LOG=debug");
}
```
Without the subscriber, log calls print nothing. `RUST_LOG=debug` or `RUST_LOG=my_app=trace` sets levels per module, like `logback.xml` loggers.

### 37. Test errors and edge cases
```rust
fn parse_age(s: &str) -> Result<u8, String> {
    let n: u8 = s.parse().map_err(|_| format!("not a number: {s}"))?;
    if n > 150 { return Err("too old".into()); }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_bad_input() {
        assert!(parse_age("abc").is_err());
        assert!(matches!(parse_age("151"), Err(e) if e == "too old"));
    }

    #[test]
    fn accepts_valid_ages() {
        for (input, want) in [("0", 0), ("42", 42), ("150", 150)] {
            assert_eq!(parse_age(input), Ok(want), "input: {input}");
        }
    }
}
```
`matches!` with a guard checks the variant *and* its content. A loop over tuples gives you JUnit's `@ParameterizedTest` without any framework.

### 38. Tests that touch the filesystem [tempfile]
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn writes_and_reads() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("out.txt");
        std::fs::write(&path, "hi")?;
        assert_eq!(std::fs::read_to_string(&path)?, "hi");
        Ok(())
    }   // dir and its contents are deleted here, when `dir` is dropped
}
```
JUnit's `@TempDir`. Cleanup happens through `Drop`, even when the assertion fails.

---


# Gotchas

The compile errors everyone hits, with the error code you'll see. Use `rustc --explain E0382` for the long version.

### 1. Using a value after moving it (E0382)
```rust
let v = vec![1, 2];
for x in v { }           // `for x in v` consumes v
println!("{}", v.len()); // ❌ borrow of moved value
```
Fix: `for x in &v`. Java habit: assuming a loop only reads.

### 2. Changing a collection while holding a reference into it (E0502)
```rust
let mut v = vec![1, 2];
let first = &v[0];
v.push(3);               // ❌ push may reallocate, which would leave `first` dangling
println!("{first}");
```
Fix: copy the value (`let first = v[0];`) or finish using `first` before `push`. Same with `for x in &v { v.push(..) }`.

### 3. Forgetting `mut` (E0596)
```rust
let v = vec![1];
v.push(2);               // ❌ cannot borrow as mutable
```
Fix: `let mut v`. `let` is stricter than `final`: the contents are frozen too.

### 4. `&str` where `String` is expected (E0308)
```rust
fn greet() -> String { "hi" }   // ❌ expected String, found &str
```
Fix: `"hi".to_string()` or `String::from("hi")`. String literals are always `&str`.

### 5. Moving out of a Vec by index (E0507)
```rust
let v = vec![String::from("a")];
let s = v[0];            // ❌ cannot move out of index
```
Fix: borrow `&v[0]`, or clone `v[0].clone()`. Moving it out would leave a hole in the Vec. (Works for `Copy` types like `i32`, which get copied instead.)

### 6. Printing a struct (E0277)
```rust
struct P { x: i32 }
println!("{}", p);       // ❌ P doesn't implement Display
```
Fix: `#[derive(Debug)]` and `{:?}`, or `impl Display` (see Structs above). No inherited `toString()`.

### 7. Sorting floats (E0277)
```rust
let mut v = vec![2.5, 1.0];
v.sort();                // ❌ f64: Ord is not satisfied
```
Floats have `NaN`, so they have no total order. Fix: `v.sort_by(|a, b| a.total_cmp(b))`.

### 8. Indexing with the wrong integer type (E0277)
```rust
let i: i32 = 0;
v[i];                    // ❌ the type [i32] cannot be indexed by i32
```
Fix: `v[i as usize]`, or declare `i` as `usize`. No implicit integer conversions, ever.

### 9. `?` in a function that returns nothing (E0277)
```rust
fn main() {
    let n: i32 = "5".parse()?;   // ❌ ? can only be used in a fn that returns Result/Option
}
```
Fix: `fn main() -> anyhow::Result<()>` and `Ok(())` at the end.

### 10. Thread closure borrowing a local (E0373)
```rust
let v = vec![1];
std::thread::spawn(|| println!("{v:?}"));   // ❌ closure may outlive v
```
Fix: `move ||`. The thread could outlive the function, so it must own its data. Or use `thread::scope`.

### 11. Borrowing from a temporary (E0716)
```rust
let parts: Vec<&str> = line().trim().split(',').collect();   // line() returns String
println!("{parts:?}");   // ❌ the String was freed at the end of the previous line
```
Fix: keep the owner alive: `let l = line(); let parts: Vec<&str> = l.trim()...`.

### 12. Reading a map entry, then writing to the map (E0502)
```rust
if let Some(v) = m.get("a") {
    m.insert("b".into(), 1);   // ❌ m is still borrowed by v
    println!("{v}");
}
```
Fix: copy the value out first: `if let Some(&v) = m.get("a")`. Or use `m.entry(..)`.

### 13. Calling `&mut self` methods while iterating your own field (E0502)
```rust
fn sum(&mut self) {
    for i in self.items.iter() {
        self.add(*i);    // ❌ self.add borrows all of self mutably, but items is being read
    }
}
```
Fix: work on fields directly (`self.total += ...`), compute first and then mutate, or split the struct. Java habit: calling helper methods freely inside loops.

### 14. Returning a reference without a lifetime (E0106)
```rust
fn longest(a: &str, b: &str) -> &str { ... }   // ❌ missing lifetime specifier
```
The compiler can't tell whether the result borrows from `a` or from `b`. Fix: `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str`, or return a `String`.

### 15. `if`/`match` branches with different types (E0308)
```rust
let y = if x > 0 { "pos" } else { 0 };   // ❌ if and else have incompatible types
```
Every branch must produce the same type. There's no `Object` to fall back to.

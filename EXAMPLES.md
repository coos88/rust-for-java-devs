# Everyday Rust

Copy-paste snippets for common tasks. Crates in brackets go in via `cargo add <crate>`.

## App skeleton [anyhow]
```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {                     // `?` works everywhere below; errors print + exit 1
    let cfg = std::fs::read_to_string("app.toml")
        .context("reading app.toml")?;        // adds context to the error message, like wrapping an exception
    println!("{cfg}");
    Ok(())
}
```

## Strings
```rust
let s = "  a,b,,c  ";
let t = s.trim();                             // "a,b,,c"
let parts: Vec<&str> = t.split(',').collect(); // ["a", "b", "", "c"]
let yes = s.contains("b") && s.starts_with("  a");
let r = s.replace(",", ";");                  // new String
let up = s.to_uppercase();
let bytes = s.len();                          // len = bytes, not chars
let chars = s.chars().count();                // chars
let n: i32 = "42".parse()?;                   // the type annotation picks the parser
let joined = vec!["a", "b"].join("-");        // "a-b"
let msg = format!("{name} is {age}", name = "x", age = 3);
if let Some((k, v)) = "key=val".split_once('=') { }
for line in text.lines() { }
```

## Collections
```rust
let mut v = vec![3, 1, 2, 1];
v.push(4);  v.pop();  v.len();  v.is_empty();  v.contains(&3);
v.sort();  v.dedup();                         // dedup removes adjacent duplicates → sort first
v.retain(|x| *x > 1);                         // removeIf, inverted
users.sort_by_key(|u| u.age);
users.sort_by(|a, b| a.name.cmp(&b.name));    // Comparator
let names: Vec<&str> = users.iter().map(|u| u.name.as_str()).collect();
let adults: Vec<&User> = users.iter().filter(|u| u.age >= 18).collect();
let total: u32 = users.iter().map(|u| u.age).sum();
let first = v.first();                        // Option<&T>, never panics

use std::collections::HashMap;
let mut m = HashMap::new();
m.insert("a", 1);
let x = m.get("a").copied().unwrap_or(0);     // get returns Option<&V>
*m.entry("b").or_insert(0) += 1;              // count
for (k, v) in &m { }                          // iteration order is random, like Java's HashMap
let m: HashMap<_, _> = vec![("a", 1), ("b", 2)].into_iter().collect();
```

## Structs you'll write daily
```rust
#[derive(Debug, Clone, Default, PartialEq)]
pub struct User { pub name: String, pub age: u32 }

impl std::fmt::Display for User {             // toString()
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

impl From<&str> for User {                    // conversion. Also gives you "x".into()
    fn from(name: &str) -> Self { User { name: name.into(), ..Default::default() } }
}

let u: User = "coos".into();
println!("{u} / {u:?}");                      // Display / Debug
```

## Files
```rust
use std::fs;
use std::path::Path;

let text = fs::read_to_string("in.txt")?;
fs::write("out.txt", "hello")?;               // creates or overwrites
fs::create_dir_all("a/b/c")?;
if Path::new("in.txt").exists() { }
let p = Path::new("dir").join("file.txt");    // Paths.get().resolve()

for entry in fs::read_dir(".")? {             // list a directory
    let path = entry?.path();
}

use std::io::{BufRead, BufReader};            // big file, line by line
for line in BufReader::new(fs::File::open("big.log")?).lines() {
    let line = line?;
}
```

## CLI args & env [clap]
```rust
let args: Vec<String> = std::env::args().collect();   // args[0] = program name
let home = std::env::var("HOME")?;                    // Err if unset
let port = std::env::var("PORT").unwrap_or("8080".into());

use clap::Parser;
#[derive(Parser)]
struct Cli {
    path: String,                             // positional
    #[arg(short, long, default_value_t = 1)]
    count: u32,                               // -c 3 / --count 3
    #[arg(long)]
    verbose: bool,                            // --verbose
}
let cli = Cli::parse();                       // --help generated for you
```

## JSON [serde (features: derive), serde_json]
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    id: u32,
    #[serde(rename = "itemName")] name: String,   // @JsonProperty
    #[serde(default)] tags: Vec<String>,          // missing in JSON → empty
}

let item: Item = serde_json::from_str(r#"{"id":1,"itemName":"x"}"#)?;
let json = serde_json::to_string_pretty(&item)?;
let any: serde_json::Value = serde_json::from_str(&json)?;   // untyped, like JsonNode
let id = any["id"].as_u64();
```

## HTTP client [ureq (features: json)] (blocking, simple)
```rust
let body: String = ureq::get("https://example.com").call()?.body_mut().read_to_string()?;
let item: Item = ureq::get("https://api.example.com/item/1").call()?.body_mut().read_json()?;
```
In async code use `reqwest` instead.

## Time [chrono]
```rust
use std::time::{Duration, Instant};
let start = Instant::now();
std::thread::sleep(Duration::from_millis(100));
println!("took {:?}", start.elapsed());       // "100.2ms"

let now = chrono::Local::now();
println!("{}", now.format("%Y-%m-%d %H:%M")); // DateTimeFormatter
let utc = chrono::Utc::now().to_rfc3339();
```

## Run a process
```rust
use std::process::Command;
let out = Command::new("git").args(["status", "--short"]).output()?;   // ProcessBuilder
println!("{}", String::from_utf8_lossy(&out.stdout));
let ok = Command::new("make").status()?.success();                    // inherit stdout, just wait
```

## Threads
```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let handles: Vec<_> = (0..4).map(|_| {
    let c = Arc::clone(&counter);             // one Arc per thread
    thread::spawn(move || *c.lock().unwrap() += 1)
}).collect();
for h in handles { h.join().unwrap(); }

let (tx, rx) = mpsc::channel();               // BlockingQueue
thread::spawn(move || tx.send("done").unwrap());
println!("{}", rx.recv()?);

thread::scope(|s| {                           // scoped threads may borrow locals, no Arc needed
    s.spawn(|| println!("{}", v.len()));
});
```

## Async [tokio (features: full)]
```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let a = tokio::spawn(async { 1 });        // CompletableFuture.supplyAsync
    let (x, y) = tokio::join!(work(1), work(2));  // run concurrently, wait for both
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("{} {x} {y}", a.await?);
    Ok(())
}
async fn work(n: u32) -> u32 { n * 2 }
```

## Logging [tracing, tracing-subscriber (features: env-filter)]
```rust
tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();                                  // once, in main. Without it nothing prints
tracing::info!(user = "coos", "logged in");   // structured field + message
tracing::warn!("disk at {}%", 91);
```
Run with `RUST_LOG=debug cargo run`.

## Custom errors [thiserror]
```rust
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("user {0} not found")]
    NotFound(u32),
    #[error("io failed")]
    Io(#[from] std::io::Error),               // #[from] lets `?` convert io::Error automatically
}
fn load(id: u32) -> Result<String, AppError> {
    let s = std::fs::read_to_string("db.txt")?;   // io::Error → AppError::Io
    if s.is_empty() { return Err(AppError::NotFound(id)); }
    Ok(s)
}
```

## Tests
```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adds() { assert_eq!(add(2, 2), 4); }
    #[test]
    fn returns_result() -> anyhow::Result<()> {   // `?` in tests
        let n: i32 = "4".parse()?;
        assert_eq!(add(2, 2), n);
        Ok(())
    }
}
```
`cargo test`, `cargo test adds` (filter by name), `cargo test -- --nocapture` (show println output).

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

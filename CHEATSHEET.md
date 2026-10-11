# Rust cheat sheet (for Java devs)

## Variables & types

| Rust | Java |
|---|---|
| `let x = 5;` | `final var x = 5;` (immutable by default) |
| `let mut x = 5;` | `var x = 5;` |
| `let x: i64 = 5;` | `long x = 5;` |
| `const MAX: u32 = 10;` | `static final int MAX = 10;` |
| `let x = x * 2;` | shadowing: new var, same name. No Java equivalent |
| `i8..i128`, `u8..u128`, `isize/usize` | signed/unsigned. `usize` = indexes/lengths |
| `f32`, `f64`, `bool`, `char` (4-byte Unicode) | |
| `()` | `void` |
| `(1, "a")` → `t.0`, `let (a, b) = t;` | tuple. No Java equivalent |
| `[i32; 3]` | fixed array |
| `x as u8` | cast (no implicit widening, ever) |

No `null`. Use `Option<T>`.

Unlike `final`, `let` also freezes the **contents**: `let v = vec![]; v.push(1)` fails. `mut` = may reassign *and* mutate in place. `&mut x` needs `let mut x`.

### Signed vs unsigned
Same bits, different range. Signed spends one bit on the sign.

| type | range | Java |
|---|---|---|
| `i8` | -128..127 | `byte` |
| `u8` | 0..255 | none (raw bytes, `&[u8]`) |
| `i32` | ±2.1 billion | `int` (the default for integer literals) |
| `u32` | 0..4.2 billion | none |
| `i64` / `u64` | ±9.2e18 / 0..1.8e19 | `long` / none |
| `usize` | 0..platform max (64-bit on your Mac) | `int` as used for indexes |

Java has no unsigned types (except `char`). Use `u*` when negative values make no sense (lengths, counts, bytes).
Overflow: **panics** in debug builds and silently wraps in release. To choose explicitly: `x.checked_add(y)` → `Option`, `wrapping_add`, `saturating_add`.

## Strings

| | |
|---|---|
| `String` | owned, growable, heap (`StringBuilder`-ish) |
| `&str` | borrowed view into string data. Literals are `&'static str` (`'static` = lives for the whole program, see Lifetimes) |
| `s.to_string()`, `String::from("x")` | `&str` → `String` |
| `&s`, `s.as_str()` | `String` → `&str` |
| `format!("{a} {}", b)` | `String.format` |
| `println!("{x:?}")` | `{:?}` = debug print, `{:#?}` = pretty |

Function params: take `&str`, return `String`. `&String` auto-converts to `&str` (and `&Vec<T>` to `&[T]`), so one `&str` param accepts both kinds.

## Control flow (everything is an expression)

| Java | Rust |
|---|---|
| `c ? a : b` | `if c { a } else { b }` |
| `if (x > 0)` | `if x > 0` (no parens, braces required) |
| `if (list)` (JS/PHP truthiness) | none. The condition must be a `bool` |
| `switch` | `match` (exhaustive, no fallthrough, no `break`) |
| `for (int i=0; i<n; i++)` | `for i in 0..n` |
| `for (var x : list)` | `for x in &list` |
| `do { } while (c)` | `loop { ...; if !c { break; } }` |
| `while (true)` | `loop` |
| `i++` | `i += 1` (no `++`/`--`) |
| `try/catch` | `match` on `Result`, or `?` |

### Blocks return values (so `if` does too)
Every `{ }` block evaluates to its last line, if that line has no `;`. A trailing `;` makes the value `()`.

```rust
let z = {              // plain block
    let a = 2;
    a * 3              // no `;` → the block's value. z = 6
};

let y = if x > 0 {     // `if` branches are blocks, so `if` has a value. This is Rust's ternary
    "pos"
} else {
    "neg"              // every branch must return the same type
};

fn double(n: i32) -> i32 {
    n * 2              // a fn body is a block too. That's why `return` is optional
}
```
In Java, `if` and `{ }` are statements and produce nothing. The nearest Java thing is a switch expression (`case 1 -> ...`, `yield`). `match` ≈ Java 21 pattern-matching `switch` on a sealed interface: exhaustive, and it yields a value.

### Loops
```rust
loop { if done() { break; } }               // infinite
let v = loop { break 42; };                 // break with a value (loop only)
while n > 0 { n -= 1; }
while let Some(top) = stack.pop() { }       // loop until the pattern stops matching

for i in 0..10 {}                           // 0..9.   0..=10 inclusive
for i in (0..10).rev() {}                   // 9..0
for i in (0..10).step_by(2) {}              // 0,2,4..
for x in &v {}                              // x: &T,     v still usable
for x in &mut v { *x += 1; }                // x: &mut T, modify in place
for x in v {}                               // x: T,      v is moved (gone after)
for (i, x) in v.iter().enumerate() {}       // index + value

'outer: for a in 0..5 {                     // labels
    for b in 0..5 {
        if b == 2 { continue 'outer; }
        if a == 3 { break 'outer; }
    }
}
```

### match
```rust
match n {
    0 => "zero",
    1 | 2 => "small",                       // or
    3..=9 => "mid",                         // range
    x if x < 0 => "neg",                    // guard
    _ => "big",                             // catch-all. Must cover every case or it won't compile
}

match point {                               // destructure
    (0, 0) => "origin",
    (_, 0) | (0, _) => "on axis",
    _ => "elsewhere",
}
match &user {                               // match on a ref: fields bind as refs
    User { age: 0..18, .. } => "minor",     // struct pattern, `..` ignores the rest
    User { name, .. } => name.as_str(),
}
match msg {
    Msg::Move { x, y } => {}                // enum variant with fields
    Msg::Write(text) => {}
    Msg::Quit => {}
}
match n { id @ 1..=9 => id, _ => 0 }        // @ binds the matched value
let ok = matches!(c, 'a'..='z' | '_');      // match as a bool
```

### if let / let else
```rust
if let Some(x) = opt { } else { }           // match one pattern, ignore the rest
if let Some(x) = opt && x > 0 { }           // let chains (edition 2024)
let Some(x) = opt else { return; };         // must match, else diverge (return/break/panic)
                                            // x stays in scope after. Great for early exits
```

### Early exit
`return` (from fn), `break`/`continue` (loops), `?` (return the error), `panic!` (crash).
Inside a closure, `return` returns from the closure, not from the outer fn.

## Functions
```rust
fn add(a: i32, b: i32) -> i32 { a + b }   // last expr = return value (no `;`)
fn f() { return; }                         // explicit return for early exit
let sq = |x: i32| x * x;                   // closure = lambda
let sq = move |x| x * n;                   // `move`: takes ownership of captured vars. Needed when the
                                           // closure outlives this scope (thread::spawn, returning it)
```
No overloading, no default args, no varargs (use macros/builders).

## Ownership (the core)
1. Each value has **one owner**. Owner goes out of scope → value dropped (destructor runs).
2. Assign/pass a non-`Copy` value = **move**. Old variable is dead.
3. Borrow with `&T` (shared, read-only, many allowed) **or** `&mut T` (exclusive, one allowed). Never both at once.
4. A reference can't outlive what it points to.

Why rule 3: it's `ConcurrentModificationException` (and data races) turned into a compile error. Nobody can change a thing while someone else is reading it.

```rust
let a = String::from("x");
let b = a;            // move. `a` unusable
let c = b.clone();    // explicit deep copy
f(&c);                // lend read-only
g(&mut d);            // lend mutable
```
`Copy` types (ints, floats, bool, char, tuples of Copy) are copied, not moved.
Method calls borrow automatically: `s.len()` doesn't move `s`.
Java analogy: every object is held by exactly one variable, and the compiler tracks which one. GC replaced by scope.

## Structs & impl (≈ class, minus inheritance)
```rust
#[derive(Debug, Clone, PartialEq)]          // auto-generate toString/clone/equals
pub struct User { pub name: String, pub age: u32 }   // fields are private unless `pub`

impl User {
    pub fn new(name: &str) -> Self { Self { name: name.into(), age: 0 } }  // "constructor" is a convention
    fn age(&self) -> u32 { self.age }        // &self  = read
    fn birthday(&mut self) { self.age += 1 } // &mut self = write
    fn into_name(self) -> String { self.name } // self = consumes object
}
let u = User::new("coos");                  // static call ::, method call .
let u2 = User { age: 3, ..u };              // struct update syntax. Moves non-Copy fields (name) out of u
struct Meters(f64);                         // tuple struct / newtype
```

## Enums (≈ sealed interface + records)
```rust
enum Shape { Circle { r: f64 }, Rect(f64, f64), Empty }
match s {
    Shape::Circle { r } => 3.14 * r * r,
    Shape::Rect(w, h) => w * h,
    Shape::Empty => 0.0,
}
```

## Option & Result (no null, no exceptions)
```rust
enum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }

let n: i32 = "5".parse()?;          // `?` = return Err early (≈ throws, but explicit)
                                    // only inside a fn returning Result/Option (main too: fn main() -> Result<(), E>)
                                    // converts the error via `From`, which is why anyhow::Error accepts any error
opt.unwrap()                        // panic if None. Prototypes/tests only
opt.expect("msg")                   // unwrap with message
opt.unwrap_or(0) / .unwrap_or_default()
opt.map(|x| x + 1) / .and_then(f) / .ok_or(err)
res.map_err(|e| ...) / .ok()
panic!("boom")                      // unrecoverable ≈ RuntimeException that you don't catch
```
Errors in apps: `anyhow` crate. In libraries: `thiserror` crate.

### Panics
`Result` = expected failure (≈ checked exception). Panic = bug or broken invariant (≈ uncaught RuntimeException).
On a panic Rust prints the message, unwinds the stack, and kills the thread. Unwinding = walk back up the call stack running every `Drop` on the way, so files and locks get released (like `finally` blocks during exception propagation). Main thread → exit code 101. `RUST_BACKTRACE=1` for a stack trace.

Triggers: `panic!`, `unwrap`/`expect` on `None`/`Err`, `v[i]` out of bounds, integer overflow (debug only), divide by zero, `todo!`/`unimplemented!`/`unreachable!`, failed `assert!`, `RefCell` double mutable borrow, slicing a `&str` mid-character.

- Spawned thread panics → `join()` returns `Err`. The program keeps running.
- `catch_unwind` exists. Use it only at FFI/thread-pool boundaries, not as try/catch.
- `panic = "abort"` in `[profile]` skips unwinding: smaller binary, no `Drop`.
- Libraries return `Result`. `unwrap` belongs in tests and prototypes. Where failure can't happen, write `expect("why")`.

## Traits (≈ interface, more powerful)
```rust
trait Speak {
    fn name(&self) -> String;
    fn hi(&self) -> String { format!("hi {}", self.name()) }  // default method
}
impl Speak for User { fn name(&self) -> String { self.name.clone() } }
impl Speak for i32 { ... }          // implement for types you didn't write
```
Common std traits: `Debug` (toString for devs), `Display` (toString for users), `Clone`, `Copy`, `PartialEq/Eq` (equals), `Hash` (hashCode), `PartialOrd/Ord` (Comparable), `Default`, `From/Into` (conversions), `Iterator`, `Drop` (destructor).

### derive: auto-implement a trait
There's no `Object` base class, so a new type can't print, compare, copy or hash itself until you implement those traits. `#[derive(...)]` asks the compiler to write the `impl` for you, field by field:

```rust
#[derive(PartialEq)]
struct Point { x: i32, y: i32 }

// generates (roughly):
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool { self.x == other.x && self.y == other.y }
}
```

| derive | gives you | Java |
|---|---|---|
| `Debug` | `{:?}` printing | IDE-generated `toString()` |
| `Clone` | `.clone()` deep copy | `clone()` / copy constructor |
| `Copy` | implicit copy instead of move (needs `Clone`, only for all-`Copy` fields) | primitives' behavior |
| `PartialEq`, `Eq` | `==`, `!=` | `equals()` |
| `Hash` | usable as a `HashMap` key (with `Eq`) | `hashCode()` |
| `PartialOrd`, `Ord` | `<`, `>`, `.sort()`: compares fields in declaration order | `Comparable` |
| `Default` | `Type::default()`: every field set to its default (0, "", false, None) | no-arg constructor |

- Every field must implement the trait too, or the derive fails (e.g. a struct holding an `f64` can't derive `Eq`).
- Crates add their own derives: `Serialize`/`Deserialize` (serde), `Parser` (clap), `Error` (thiserror).
- Need custom behavior (e.g. `PartialEq` that ignores one field)? Skip the derive and write the `impl` by hand.
- `cargo install cargo-expand` → `cargo expand` shows the generated code.

## Generics
```rust
fn biggest<T: PartialOrd>(xs: &[T]) -> &T { ... }
fn show(x: impl Display) {}               // shorthand for a generic, not dyn
fn f<T>(x: T) where T: Clone + Debug {}   // where clause
struct Wrapper<T> { v: T }
```
Monomorphized: one copy of code per concrete type (fast, no erasure, bigger binaries).
`Box<dyn Trait>` = runtime polymorphism (≈ Java's interface reference, vtable).

Generics vs `dyn`: `fn f<T: Trait>(xs: &[T])` gets one concrete `T` per call, so all elements have the same type. `&[Box<dyn Trait>]` can mix types, resolved at runtime. Use generics by default and `dyn` for mixed collections.

## Collections

| Rust | Java |
|---|---|
| `Vec<T>`, `vec![1,2]` | `ArrayList` |
| `HashMap<K,V>` | `HashMap` |
| `HashSet<T>` | `HashSet` |
| `BTreeMap`, `BTreeSet` | `TreeMap`, `TreeSet` |
| `VecDeque` | `ArrayDeque` |
| `&[T]` slice | view into array/Vec |

`map.entry(k).or_insert(0)` ≈ `computeIfAbsent`. It returns `&mut V`, so a counter is `*map.entry(k).or_insert(0) += 1`. `v[i]` panics out of bounds. `v.get(i)` → `Option`.

## Iterators (≈ Streams, lazy, zero-cost)
```rust
v.iter()        // yields &T
v.iter_mut()    // yields &mut T
v.into_iter()   // yields T, consumes v
v.iter().filter(|x| **x > 1).map(|x| x * 2).collect::<Vec<_>>();
                // iter() yields &T, filter passes a ref to that: &&T, hence **
v.iter().copied()   // &T → T (Copy types). .cloned() for Clone types
.sum() .count() .max() .max_by_key(f) .min_by_key(f) .any(f) .all(f) .find(f) .fold(0, f)
.enumerate() .zip(other) .rev() .take(n) .skip(n) .chain(o) .flat_map(f)
```

## Modules & visibility
```rust
mod net;              // loads net.rs or net/mod.rs. A file is only compiled if a `mod` line names it
use std::collections::HashMap;
use crate::net::Client;   // crate:: = project root, super:: = parent
pub fn x() {}         // public. Default is private (to the module)
pub(crate) fn y() {}  // package-private-ish
```
crate = compilation unit (jar). Module = package. File = module, *once declared with `mod`*.

## Smart pointers
`Rc`/`Arc` give shared ownership, but only `&T` (read-only) because of ownership rule 3. `RefCell`/`Mutex` add mutation back, checked at runtime (within one thread / across threads). Hence `Arc<Mutex<T>>`.

| | |
|---|---|
| `Box<T>` | heap allocate, single owner |
| `Rc<T>` | shared ownership, refcounted, single thread |
| `Arc<T>` | `Rc` but thread-safe |
| `RefCell<T>` | mutate through shared ref, checked at runtime |
| `Mutex<T>`, `RwLock<T>` | thread-safe mutation. Data lives *inside* the lock |

Typical shared mutable state: `Arc<Mutex<T>>`.

## Lifetimes
```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
```
Not a duration. A label telling the compiler "the output borrows from these inputs". `<'a>` declares the label, like `<T>`.
You only write them when a fn returns a reference and takes more than one reference, or a struct holds a reference (`struct View<'a> { s: &'a str }`). Everywhere else they're inferred. Error E0106 "missing lifetime specifier" = add one, or return an owned value.
`'static` = lives for the whole program.

## Concurrency
```rust
std::thread::spawn(move || { ... }).join();
let (tx, rx) = std::sync::mpsc::channel();
```
`Send` = may move to another thread. `Sync` = may be shared between threads. Compiler enforces both: no data races.
`Rc` and `RefCell` are neither, which is the compiler forcing you to `Arc`/`Mutex` when a value crosses threads. The error says "cannot be sent between threads safely".
Async: `async fn`, `.await`, needs a runtime (`tokio`). `Future` ≈ `CompletableFuture`, but lazy.

## Macros
`name!(...)`. Code generation at compile time. `println!`, `vec!`, `format!`, `assert_eq!`, `todo!()`, `unimplemented!()`, `dbg!(x)`.

## Attributes (≈ annotations)
`#[attr]` applies to the next item. `#![attr]` (with `!`) applies to the enclosing file/module/crate, e.g. at the top of `main.rs`.

**Key difference:** attributes act only at compile time. They generate code, toggle code, or configure the compiler. There's no runtime reflection, so nothing can "scan for" an attribute while the program runs. That's why Spring-style frameworks don't exist.

| Rust | Java | what |
|---|---|---|
| `#[derive(Debug, Clone, PartialEq)]` | Lombok `@ToString @EqualsAndHashCode` | generate trait impls (see Traits → derive) |
| `#[test]`, `#[should_panic]` | `@Test`, `assertThrows` | mark tests |
| `#[cfg(test)]` | none | compile this only for `cargo test` |
| `#[cfg(target_os = "macos")]` | none (runtime `if`) | compile only on that platform. The code doesn't exist elsewhere |
| `#[cfg(feature = "json")]` | none | compile only when a Cargo feature is on |
| `#[allow(dead_code)]`, `#[deny(..)]` | `@SuppressWarnings` | tune lints |
| `#[deprecated(note = "use y")]` | `@Deprecated` | warn callers |
| `#[must_use]` | `@CheckReturnValue` | warn if the result is ignored. `Result` has it built in |
| `#[non_exhaustive]` | none | others can't exhaustively match your enum, so you can add variants later |
| `#[inline]` | none (JIT decides) | hint to inline across crates |
| `#[repr(C)]` | none | C memory layout, for FFI |
| `/// docs` | `/** javadoc */` | sugar for `#[doc = "..."]`. Markdown, shown by `cargo doc` |

From crates (procedural macros, run by the compiler):
```rust
#[tokio::main]                        // rewrites async main into a runtime setup
#[derive(Serialize, Deserialize)]     // serde generates (de)serialization code
#[serde(rename = "itemName")]         // field-level config, read by serde's derive
#[derive(clap::Parser)]               // CLI parser from a struct
```
Writing your own attribute = a procedural macro in a separate crate (`syn` + `quote`). Far heavier than declaring a Java `@interface`, and rarely needed.

## Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn works() { assert_eq!(2 + 2, 4); }
    #[test] #[should_panic] fn boom() { panic!() }
}
```
`cargo test`. Tests sit in the same file as the code. Integration tests go in `tests/`.

## Misc
- `unsafe { }`: raw pointers, FFI. Rare.
- `_` ignores a value. `_x` silences the unused warning.
- `impl Trait for Type` can live in your crate only if you own the trait **or** the type (orphan rule).
- `Self` = current type. `self` = current instance.
- `todo!()` compiles anywhere, panics at runtime. Handy for stubs.

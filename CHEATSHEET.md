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

## Strings
| | |
|---|---|
| `String` | owned, growable, heap (`StringBuilder`-ish) |
| `&str` | borrowed view into string data. Literals are `&'static str` |
| `s.to_string()`, `String::from("x")` | `&str` → `String` |
| `&s`, `s.as_str()` | `String` → `&str` |
| `format!("{a} {}", b)` | `String.format` |
| `println!("{x:?}")` | `{:?}` = debug print, `{:#?}` = pretty |

Function params: take `&str`, return `String`.

## Control flow (everything is an expression)
```rust
let y = if x > 0 { "pos" } else { "neg" };   // no ternary, this is it
let v = loop { break 42; };                  // loop returns a value
while n > 0 { n -= 1; }
for i in 0..10 {}      // 0..=10 inclusive
for x in &vec {}       // borrow elements
'outer: for ... { break 'outer; }
match n {
    0 => "zero",
    1 | 2 => "small",
    3..=9 => "mid",
    x if x < 0 => "neg",   // guard
    _ => "big",            // default; match must be exhaustive
}
if let Some(x) = opt { }
let Some(x) = opt else { return; };          // early exit
```

## Functions
```rust
fn add(a: i32, b: i32) -> i32 { a + b }   // last expr = return value (no `;`)
fn f() { return; }                         // explicit return for early exit
let sq = |x: i32| x * x;                   // closure = lambda
let sq = move |x| x * n;                   // `move`: takes ownership of captured vars
```
No overloading, no default args, no varargs (use macros/builders).

## Ownership (the core)
1. Each value has **one owner**. Owner goes out of scope → value dropped (destructor runs).
2. Assign/pass a non-`Copy` value = **move**. Old variable is dead.
3. Borrow with `&T` (shared, read-only, many allowed) **or** `&mut T` (exclusive, one allowed). Never both at once.
4. A reference can't outlive what it points to.

```rust
let a = String::from("x");
let b = a;            // move. `a` unusable
let c = b.clone();    // explicit deep copy
f(&c);                // lend read-only
g(&mut d);            // lend mutable
```
`Copy` types (ints, floats, bool, char, tuples of Copy) are copied, not moved.
Java analogy: every object is a unique `final` pointer the compiler tracks. GC replaced by scope.

## Structs & impl (≈ class, minus inheritance)
```rust
#[derive(Debug, Clone, PartialEq)]          // auto-generate toString/clone/equals
pub struct User { pub name: String, age: u32 }

impl User {
    pub fn new(name: &str) -> Self { Self { name: name.into(), age: 0 } }  // "constructor" is a convention
    fn age(&self) -> u32 { self.age }        // &self  = read
    fn birthday(&mut self) { self.age += 1 } // &mut self = write
    fn into_name(self) -> String { self.name } // self = consumes object
}
let u = User::new("coos");                  // static call ::, method call .
let u2 = User { age: 3, ..u };              // struct update syntax
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
opt.unwrap()                        // panic if None. Prototypes/tests only
opt.expect("msg")                   // unwrap with message
opt.unwrap_or(0) / .unwrap_or_default()
opt.map(|x| x + 1) / .and_then(f) / .ok_or(err)
res.map_err(|e| ...) / .ok()
panic!("boom")                      // unrecoverable ≈ RuntimeException that you don't catch
```
Errors in apps: `anyhow` crate. In libraries: `thiserror` crate.

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

## Generics
```rust
fn biggest<T: PartialOrd>(xs: &[T]) -> &T { ... }
fn show(x: impl Display) {}               // shorthand
fn f<T>(x: T) where T: Clone + Debug {}   // where clause
struct Wrapper<T> { v: T }
```
Monomorphized: one copy of code per concrete type (fast, no erasure, bigger binaries).
`Box<dyn Trait>` = runtime polymorphism (≈ Java's interface reference, vtable).

## Collections
| Rust | Java |
|---|---|
| `Vec<T>`, `vec![1,2]` | `ArrayList` |
| `HashMap<K,V>` | `HashMap` |
| `HashSet<T>` | `HashSet` |
| `BTreeMap`, `BTreeSet` | `TreeMap`, `TreeSet` |
| `VecDeque` | `ArrayDeque` |
| `&[T]` slice | view into array/Vec |

`map.entry(k).or_insert(0)` ≈ `computeIfAbsent`. `v[i]` panics out of bounds. `v.get(i)` → `Option`.

## Iterators (≈ Streams, lazy, zero-cost)
```rust
v.iter()        // yields &T
v.iter_mut()    // yields &mut T
v.into_iter()   // yields T, consumes v
v.iter().filter(|x| **x > 1).map(|x| x * 2).collect::<Vec<_>>();
.sum() .count() .max() .min_by_key(f) .any(f) .all(f) .find(f) .fold(0, f)
.enumerate() .zip(other) .rev() .take(n) .skip(n) .chain(o) .flat_map(f)
```

## Modules & visibility
```rust
mod net;              // loads net.rs or net/mod.rs
use std::collections::HashMap;
use crate::net::Client;   // crate:: = project root, super:: = parent
pub fn x() {}         // public. Default is private (to the module)
pub(crate) fn y() {}  // package-private-ish
```
crate = compilation unit (jar). Module = package. File = module.

## Smart pointers
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
Not a duration. A label telling the compiler "the output borrows from these inputs". Usually inferred. `'static` = lives for the whole program.

## Concurrency
```rust
std::thread::spawn(move || { ... }).join();
let (tx, rx) = std::sync::mpsc::channel();
```
`Send` = may move to another thread. `Sync` = may be shared between threads. Compiler enforces both: no data races.
Async: `async fn`, `.await`, needs a runtime (`tokio`). `Future` ≈ `CompletableFuture`, but lazy.

## Macros
`name!(...)`. Code generation at compile time. `println!`, `vec!`, `format!`, `assert_eq!`, `todo!()`, `unimplemented!()`, `dbg!(x)`.
`#[derive(...)]` and `#[attr]` ≈ annotations, but they generate code at compile time.

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

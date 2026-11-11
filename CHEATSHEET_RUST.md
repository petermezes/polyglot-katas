# Cheat sheet: Rust for JS/TS developers

Target version: **Rust 1.85+, edition 2024** (as in `Cargo.toml`). Companion cheat sheets: [CHEATSHEET_GO.md](CHEATSHEET_GO.md), [CHEATSHEET_CSHARP.md](CHEATSHEET_CSHARP.md), [CHEATSHEET_PYTHON.md](CHEATSHEET_PYTHON.md).

---

## 0. Three things that change everything

1. **Every value has exactly one owner.** Assigning or passing a `String`, `Vec` or struct *moves* it: the old variable can't be used any more. There's no garbage collector; the value is freed when its owner goes out of scope.
2. **You lend values out instead of copying them.** `&T` is a shared, read-only borrow (any number at once), `&mut T` an exclusive, writable one (exactly one, and no `&T` alongside). The compiler checks this ("the borrow checker"), so there are no data races and no dangling pointers.
3. **No `null`, no exceptions.** "Maybe nothing" is `Option<T>` (`Some(x)` / `None`), "maybe an error" is `Result<T, E>` (`Ok(x)` / `Err(e)`). The compiler makes you handle both.

---

## 1. Variables, types, basic syntax

```rust
let x = 5;                  // immutable by default ≈ const
let mut count = 0;          // mutable ≈ let
count += 1;
let x = x * 2;              // shadowing: a new variable with the same name (allowed, common)

let n: i64 = 42;            // i8..i128, u8..u128, isize/usize (indexes and lengths are usize)
let f: f64 = 1.5;
let ok: bool = true;
let c: char = 'ô';          // one Unicode scalar value, 4 bytes
let t: (i32, &str) = (1, "a");          // tuple; t.0, t.1
let arr: [u8; 3] = [1, 2, 3];           // fixed-size array, the size is part of the type

let size = if n > 10 { "big" } else { "small" };   // if is an expression (no ternary operator)

fn add(a: i32, b: i32) -> i32 {
    a + b                   // the last expression without `;` is the return value
}

for i in 0..3 {}            // 0, 1, 2 (0..=3 includes 3)
for x in &arr {}            // iterate by reference
while count < 10 { count += 1; }
let found = loop { break 42; };          // loop can return a value

println!("{x} and {}", n + 1);           // macros end with `!`; {x} = inline variable
let s = format!("{f:.1} {t:?}");         // :.1 = one decimal place, :? = Debug output
```

- No implicit numeric conversions: `n as f64`, `i as usize`. `as` truncates silently, `u8::try_from(n)` checks.
- Integer overflow panics in debug builds and wraps in release. Explicit: `checked_add` (→ `Option`), `wrapping_add`, `saturating_sub`.
- Integer division truncates: `5 / 2 == 2`.

---

## 2. Ownership, moving, borrowing

```rust
let a = String::from("hi");
let b = a;                  // move: `a` is no longer usable
// println!("{a}");         // error: borrow of moved value: `a`
let c = b.clone();          // an explicit deep copy

let n = 5;
let m = n;                  // i32 is Copy: copied, `n` still usable

fn len(s: &String) -> usize { s.len() }        // borrows, the caller keeps ownership
fn push(v: &mut Vec<i32>) { v.push(1); }       // borrows mutably
fn consume(s: String) {}                        // takes ownership

let mut v = vec![1, 2];
let first = &v[0];          // shared borrow...
// v.push(3);               // error: can't borrow `v` as mutable, `first` is still in use
println!("{first}");        // ...ends after its last use
v.push(3);                  // now OK
```

| JS/TS | Rust |
|---|---|
| objects passed by reference, shared freely | moved by default, shared via `&T`, changed via `&mut T` |
| `structuredClone(x)` | `x.clone()` (needs `Clone`) |
| GC frees memory | freed when the owner goes out of scope (`Drop`) |
| copying a number | the same: `Copy` types (numbers, `bool`, `char`, `&T`, tuples of `Copy`) |

**Lifetimes** (`'a`) name how long a borrow lives. Usually the compiler infers them ("lifetime elision"): `fn first(xs: &[String]) -> &str` returns something borrowed from `xs`. You write them only when it's ambiguous, e.g. two input references: `fn longer<'a>(a: &'a str, b: &'a str) -> &'a str`.

---

## 3. Strings

| type | what it is | ≈ |
|---|---|---|
| `String` | owned, growable, UTF-8 | a JS string you own |
| `&str` | a borrowed view into UTF-8 text (a literal `"abc"` is `&'static str`) | a read-only slice |

```rust
let s: String = "kôň".to_string();       // or String::from, .to_owned()
let r: &str = &s;                        // &String coerces to &str
s.len();                                 // 5: bytes, not characters!
s.chars().count();                       // 3
s.to_lowercase(); s.trim(); s.split(',');  s.split_once('=');
s.starts_with("k"); s.contains("ô"); s.replace("ô", "o");
let joined = ["a", "b"].join(", ");
let text = format!("{s}-{}", 1);         // ≈ template literals
// &s[0..2]                              // byte indexes: panics if not on a char boundary
```

- Take `&str` as a parameter, return `String` when you create new text.
- Comparison (`==`, `<`, `sort()`) is byte-wise (ordinal), never culture-dependent.
- `{}` uses `Display` (for users), `{:?}` uses `Debug` (for developers, `derive(Debug)`). `{:?}` on a string prints it in quotes.

---

## 4. Collections

```rust
let mut v: Vec<i32> = vec![3, 1, 2];     // ≈ JS Array
v.push(4); v.pop();                      // pop → Option<i32>
v.len(); v.is_empty(); v[0];             // v[10] panics; v.get(10) → None
v.sort(); v.sort_by(|a, b| b.cmp(a)); v.sort_by_key(|x| x.abs());
v.dedup(); v.truncate(2); v.contains(&3);
let slice: &[i32] = &v[1..];             // a borrowed view, no copy

use std::collections::{HashMap, HashSet, BTreeMap};
let mut m: HashMap<String, i32> = HashMap::new();   // ≈ JS Map
m.insert("a".to_string(), 1);
m.get("a");                              // → Option<&i32>
m["a"];                                  // panics if missing
m.contains_key("a"); m.remove("a");
*m.entry("b".to_string()).or_insert(0) += 1;        // count: insert 0 if missing, then +1
groups.entry(len).or_default().push(word);          // group (HashMap<usize, Vec<String>>): empty Vec if missing
for (k, v) in &m {}                      // random order; BTreeMap is sorted by key
```

- `HashMap` iteration order is random (like Go). Need it sorted: `BTreeMap`, or collect into a `Vec` and sort.
- `or_insert(expr)` evaluates `expr` every time. `or_insert_with(|| expr)` only when the key is new.

---

## 5. Iterators ≈ JS Array methods, but lazy

```rust
let total: i32 = v.iter()                // iter() → &T, iter_mut() → &mut T, into_iter() → T (consumes v)
    .filter(|x| **x > 1)
    .map(|x| x * 10)
    .sum();
let names: Vec<String> = users.iter().map(|u| u.name.clone()).collect();
```

| JS | Rust |
|---|---|
| `arr.map(f)` | `iter().map(f)` … `.collect::<Vec<_>>()` |
| `arr.filter(f)` | `iter().filter(f)` (the closure gets `&&T` from `iter()`) |
| `arr.reduce(f, init)` | `iter().fold(init, f)` |
| `arr.find(f)` | `iter().find(f)` → `Option` |
| `arr.some(f)` / `every(f)` | `any(f)` / `all(f)` |
| `arr.forEach(f)` | `for x in &arr { ... }` |
| `arr.slice(0, n)` | `iter().take(n)` or `&arr[..n]` |
| `arr.entries()` | `iter().enumerate()` |
| `Math.max(...arr)` | `iter().max()` → `Option` |
| `arr.length` after filter | `.count()` |
| `Object.fromEntries` | `.collect::<HashMap<_, _>>()` |

**Difference from JS: iterators are lazy.** `map`/`filter` do nothing until something consumes them (`collect`, `sum`, `for`, `count`). An unconsumed `map(...)` is a compiler warning.

Your own iterator = a struct with `next()`; all the other methods come for free:

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
let v: Vec<u32> = Countdown(3).collect();   // [3, 2, 1]
```

---

## 6. Structs, impl, traits (instead of classes)

```rust
#[derive(Debug, Clone, PartialEq, Default)]  // generated: {:?}, .clone(), ==, Default::default()
pub struct Lane {
    pub name: String,         // pub = visible outside the module
    state: State,             // private to the module
}

impl Lane {
    pub fn new(name: &str) -> Self {                   // a convention, not a constructor keyword
        Lane { name: name.to_string(), state: State::Open }
    }
    pub fn state(&self) -> State { self.state }        // &self: reads
    pub fn close(&mut self) { self.state = State::Closed; }   // &mut self: changes
    pub fn into_name(self) -> String { self.name }     // self: consumes the Lane
}

let lane = Lane { state: State::Closed, ..Lane::new("a") };   // struct update ≈ { ...obj, state }
```

**Traits ≈ interfaces**, implemented explicitly (like C#, unlike Go):

```rust
pub trait Sensor {
    fn id(&self) -> &str;
    fn describe(&self) -> String { format!("sensor {}", self.id()) }   // default method
}
impl Sensor for CoSensor { fn id(&self) -> &str { &self.id } }

fn worst(sensors: &[Box<dyn Sensor>]) {}       // dynamic dispatch: mixed types in one list
fn print_it(s: &impl Sensor) {}                // static dispatch (generic), one type per call
fn print_it2<S: Sensor>(s: &S) {}              // the same, written as a generic
```

- No inheritance. Shared behavior = traits with default methods, shared data = composition.
- Commonly derived/implemented traits: `Debug`, `Clone`, `Copy`, `PartialEq`/`Eq`, `PartialOrd`/`Ord`, `Hash`, `Default`, `Display` (by hand), `From`/`Into`.
- `#[derive(Default)]` on a generic `Foo<T>` requires `T: Default`. Implement it by hand if that's too strict.

---

## 7. Enums and pattern matching

Rust enums can carry data in each variant (≈ TS discriminated unions, checked by the compiler):

```rust
enum Shape {
    Circle { r: f64 },
    Rect(f64, f64),
    Empty,
}

fn area(s: &Shape) -> f64 {
    match s {                                   // must cover every variant
        Shape::Circle { r } => std::f64::consts::PI * r * r,
        Shape::Rect(w, h) if w == h => w * w,   // a guard
        Shape::Rect(w, h) => w * h,
        Shape::Empty => 0.0,
    }
}

match (state, action) {                         // match on a tuple
    (Open | Restricted, Close) => Closed,       // `|` combines patterns
    _ => state,                                 // `_` = anything else
}

if let Some(x) = map.get("a") { /* x: &V */ }
let Some((key, value)) = line.split_once('=') else { return Err(...) };   // let-else
let is_rect = matches!(s, Shape::Rect(..));
```

- An enum value can only ever be one of its variants: no "unknown value 42" branch like Go's `iota` or C# enums need.
- Add a variant and every `match` without `_` stops compiling until you handle it.

---

## 8. Errors (instead of try/catch)

```rust
fn get_port(cfg: &HashMap<String, String>) -> Result<u16, ConfigError> {
    let raw = cfg.get("port").ok_or(ConfigError::Missing)?;         // Option → Result, ? returns early
    let port = raw.parse::<u16>().map_err(ConfigError::BadNumber)?; // wrap the error
    Ok(port)
}

match get_port(&cfg) {
    Ok(p) => println!("{p}"),
    Err(e) => eprintln!("{e}"),
}
```

| JS/TS | Rust |
|---|---|
| `throw new Error(...)` | `return Err(...)` |
| `try { a(); b(); } catch` | `a()?; b()?;` and handle the `Result` once, at the caller |
| `err.cause` | `Error::source()` |
| `x ?? default` | `opt.unwrap_or(default)`, `unwrap_or_else(\|\| ...)`, `unwrap_or_default()` |
| `x?.y` | `opt.map(\|x\| x.y)`, `opt.and_then(...)` |
| assertion that can't fail | `.expect("why it can't fail")` (panics otherwise) |

A custom error = an enum + `Display` + `impl Error`:

```rust
#[derive(Debug)]
pub enum ConfigError { Missing, BadNumber(std::num::ParseIntError) }

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Missing => write!(f, "missing port"),
            ConfigError::BadNumber(_) => write!(f, "invalid port"),   // the cause is in source()
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self { ConfigError::BadNumber(e) => Some(e), _ => None }
    }
}
```

- `?` also converts the error if `impl From<Inner> for YourError` exists.
- `panic!` (and `unwrap()` on `None`/`Err`) is for bugs, not for expected failures. It unwinds the thread, there's no `catch` in normal code.
- In applications the crates `anyhow` (one error type for everything) and `thiserror` (derive `Display`/`Error`) save the boilerplate.

---

## 9. Closures and generics

```rust
let factor = 3;
let triple = |x: i32| x * factor;            // borrows `factor`
let mut calls = 0;
let mut count = || calls += 1;               // borrows `calls` mutably → FnMut
let owned = move || println!("{factor}");    // `move`: takes ownership of what it captures

fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
fn apply2(f: impl Fn(i32) -> i32, x: i32) -> i32 { f(x) }   // the same, shorter

fn largest<T: PartialOrd + Copy>(xs: &[T]) -> Option<T> { ... }
fn sum<T>(xs: &[T]) -> T where T: Copy + Default + std::ops::Add<Output = T> { ... }
```

| trait | the closure may | can be called |
|---|---|---|
| `Fn` | only read what it captured | many times (from several threads too, if it's `Sync`) |
| `FnMut` | change what it captured | many times, one call at a time |
| `FnOnce` | move what it captured out | once |

Generics are compiled per type (like C++ templates, unlike Java's erasure): no runtime cost. Bounds say what a type must *be able to do* (`T: Add`), not which types are allowed (Go's `~int | ~float64`).

---

## 10. Smart pointers and shared state

| type | what for | ≈ |
|---|---|---|
| `Box<T>` | a value on the heap; `Box<dyn Trait>` for trait objects | a plain object reference |
| `Rc<T>` | several owners, one thread (read-only) | a shared reference |
| `Arc<T>` | several owners across threads (atomic counter) | a shared reference |
| `RefCell<T>` | change through `&` in one thread, checked at runtime | |
| `Mutex<T>` / `RwLock<T>` | change through `&` across threads | a lock that owns its data |
| `Arc<Mutex<T>>` | the usual "shared mutable state between threads" | |

```rust
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
let c = Arc::clone(&counter);               // a new pointer, not a copy of the data
*c.lock().unwrap() += 1;                    // the guard unlocks when dropped
```

The data lives **inside** the `Mutex`: there's no way to reach it without locking. `std::sync::Mutex` isn't reentrant: locking it twice in one thread deadlocks.

---

## 11. Concurrency

```rust
use std::thread;
use std::sync::mpsc;

// Scoped threads: may borrow local data, all are joined when the scope ends.
let data = vec![1, 2, 3];
thread::scope(|s| {
    for x in &data {
        s.spawn(move || println!("{x}"));
    }
});

// Channels ≈ Go channels (multi-producer, single-consumer).
let (tx, rx) = mpsc::channel();
for i in 0..3 {
    let tx = tx.clone();
    thread::spawn(move || tx.send(i).unwrap());    // spawn needs owned ('static) data
}
drop(tx);                                          // otherwise the loop below never ends
for value in rx { println!("{value}"); }
```

- **`Send`**: a value may be moved to another thread. **`Sync`**: a `&T` may be shared between threads. The compiler checks both. `Rc` is neither, `RefCell` is `Send` but not `Sync`, `Arc` and `Mutex` are both (when their contents are `Send`).
- Atomics: `AtomicUsize`, `AtomicBool` with `fetch_add`, `fetch_max`, `compare_exchange`, `load`/`store`.
- There's no built-in cancellation (no `AbortController` / `context`): share an `AtomicBool` flag and check it.
- **async/await** exists, but std has no runtime. For I/O-heavy code use a crate like `tokio`; `async fn` ≈ a JS async function, `.await` ≈ `await`, but a future does nothing until it is awaited (unlike a JS Promise, which starts immediately).

---

## 12. Project and tools

| JS/TS | Rust |
|---|---|
| `package.json` | `Cargo.toml` |
| `npm` / npmjs.com | `cargo` / crates.io |
| `node_modules` | `~/.cargo/registry` + `target/` |
| `package-lock.json` | `Cargo.lock` |
| `npm run build` | `cargo build` (`--release` for an optimized build) |
| `node app.js` | `cargo run` |
| `npx jest` | `cargo test` |
| `prettier` | `cargo fmt` |
| `eslint` | `cargo clippy` |
| `tsc --noEmit` | `cargo check` (fastest feedback) |

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 2), 4);
        assert!(add(1, 1) > 0, "custom message {}", 42);
    }
}
```

- Unit tests live next to the code in a `#[cfg(test)] mod tests`. Integration tests (only the public API) are files in `tests/`, each its own crate. This repo uses the latter.
- `cargo test name_part` runs matching tests, `-- --nocapture` shows `println!` output.
- Modules: a file `src/foo.rs` + `mod foo;` in `lib.rs`/`main.rs`. `pub` makes items visible outside the module, `use` imports.
- `todo!()` compiles anywhere a value is expected and panics at runtime: handy for skeletons. (Not as the body of a function returning `impl Trait`: there it has no concrete type to stand for.)

---

## 13. Common traps (summary)

- `s.len()` counts bytes. Characters: `s.chars().count()`. `&s[a..b]` panics off a char boundary.
- Assigning or passing a `String`/`Vec`/struct moves it. Use `&` to lend, `.clone()` to copy.
- `&mut` is exclusive: you can't keep a reference into a `Vec` and `push` to it at the same time.
- `v[i]` and `map[key]` panic when missing. Use `get()` for an `Option`.
- `unwrap()` is a panic waiting to happen. In real code: `?`, `expect("reason")`, or handle the case.
- Iterators are lazy. Nothing happens without `collect`/`for`/`sum`/...
- `HashMap` order is random.
- `split` keeps empty pieces; `split_whitespace` doesn't.
- `dedup` removes only neighbouring duplicates: `sort` first.
- `max_by_key`/`min_by_key` return the last/first element on ties respectively.
- `#[derive(Default)]` (and `Clone`) on `Foo<T>` adds a `T: Default` (`T: Clone`) bound.
- `as` between numeric types truncates silently. Integer overflow panics in debug only.
- `std::sync::Mutex` isn't reentrant. Don't hold a guard across a call that locks again.
- Forgetting to `drop` a channel's `Sender` makes `for x in rx` wait forever.

# Exercises

The exact behavior of each function is in the doc comments in the skeletons. Here is an overview of what each exercise practices and what is worth watching out for.

---

## 01 · Collections: word frequency

**Task:** Split text into words, count occurrences, pick the N most frequent and group words by length.

| | You practice | Trap |
|---|---|---|
| **Go** | slices, maps, `slices.SortFunc`, `cmp.Compare`, `strings.FieldsFunc` | `len("kôň")` is 5 bytes, not 3 characters. Map iteration order is random. |
| **C#** | `List`, `Dictionary`, LINQ (`GroupBy`, `OrderBy…ThenBy`, `ToDictionary`) | Default string comparison depends on the culture. Use `StringComparer.Ordinal` for deterministic sorting. |
| **Python** | `Counter`, `defaultdict`, comprehensions, `sorted(key=...)` with a tuple as the key | `sorted()` returns a new list, `list.sort()` sorts in place and returns `None`. `Counter.most_common` doesn't sort ties alphabetically. |
| **Rust** | `Vec`, `HashMap` and the entry API, `sort_by` + `Ordering::then_with`, `String` vs `&str`, `chars()` | `"kôň".len()` is 5 bytes, like in Go. `split` keeps empty pieces between neighbouring separators. `dedup` removes only neighbouring duplicates, so sort first. |

**Compare:** In Go you write the loop and the sorting by hand, in C# it's a chain of LINQ calls, in Python a comprehension and `sorted` with a key. Rust sits in between: a plain loop with `entry(...).or_insert(0)`, or an iterator chain. Which version reads best to you?

---

## 02 · Errors: config parser

**Task:** Read `key = value` line by line and return a meaningful error with the line number when something is wrong.

| | You practice | Trap |
|---|---|---|
| **Go** | errors as values, sentinel errors, a custom error type, `fmt.Errorf("%w")`, `errors.Is` / `errors.As` | Return your custom error as a pointer (`&SyntaxError{}`), otherwise `errors.As` with `*SyntaxError` won't find a match. |
| **C#** | custom exceptions, `InnerException`, the Try pattern (`TryGetInt` with `out`) | Don't catch `Exception` everywhere, only where you can wrap or handle it. |
| **Python** | an exception hierarchy with a common ancestor, attributes on exceptions, `raise ... from err` / `from None`, `str.partition`, `pytest.raises` | Don't name your class `SyntaxError`, you'd shadow the built-in exception. A bare `except:` also catches Ctrl+C. `int(" 42 ")` and `int("4_2")` both pass, Go's `Atoi` rejects them. |
| **Rust** | `Result`, one error `enum` with data in its variants, `Display` + `Error::source`, `?`, `ok_or_else` / `map_err`, `let ... else` | Don't print the cause in `Display` when `source()` returns it, error reporters would print it twice. `"42".parse::<i64>()` rejects `" 42"` and `"4_2"` like Go, but accepts `"+42"`. |

**Compare:** Go `%w` vs. C# `new XException(msg, inner)` vs. Python `raise X(...) from err` vs. Rust `source()`. Go `errors.Is` vs. C# and Python `catch` / `except` on a common ancestor vs. Rust `matches!(err, ConfigError::EmptyKey { .. })`. C# `TryGetInt(out)` vs. Python `int | None` vs. Rust `get_int(...).ok()`. Rust's `?` is Go's `if err != nil { return err }` in one character.

---

## 03 · Interfaces: tunnel sensors

**Task:** Three sensor types (CO, visibility, smoke) that can evaluate their severity, and functions over a list of sensors.

| | You practice | Trap |
|---|---|---|
| **Go** | implicit interfaces, an enum via `iota`, `fmt.Stringer`, type switch, `var _ I = T{}` | With a value receiver, both `T` and `*T` implement the interface. With a pointer receiver, only `*T` does. |
| **C#** | explicit interfaces, positional records, switch expressions, property patterns | `{x:F1}` prints a decimal comma in SK/NO culture. |
| **Python** | `typing.Protocol` (structural, like Go), `@dataclass(frozen=True)`, `IntEnum`, `__str__`, `match` with class patterns | `case OK:` compares nothing, it captures everything into a variable `OK`. Write constants dotted (`case Severity.OK:`). Nobody checks a Protocol at runtime, `@runtime_checkable` only verifies that the members exist. |
| **Rust** | traits, trait objects (`Box<dyn Sensor>`), default methods, `derive(PartialOrd, Ord)` on an enum, `Display`, `..Default::default()`, a returned `Vec<&str>` (lifetime elision) | There is no type switch: a `dyn Sensor` doesn't reveal its type. The behavior goes into the trait (open set) or you use an `enum` + `match` (closed set). A `Vec<T>` holds one type, a mixed list needs `Vec<Box<dyn Sensor>>`. |

**Compare:** The tests add their own `fakeSensor` / `FakeSensor`. In Go and Python it doesn't even know about the interface. In C# and Rust it has to declare it, and in Rust it gets `describe()` for free from the trait's default method.

---

## 04 · Generics: a stack and functional helpers

**Task:** A generic stack, `Map` / `Filter` / `Reduce`, a sum of any numbers and `MaxBy`.

| | You practice | Trap |
|---|---|---|
| **Go** | type parameters, `any`, `comparable`, `cmp.Ordered`, a custom constraint with `~` | The zero value of `Stack[T]` must work without a constructor. You get the zero value of T via `var zero T`. |
| **C#** | generic classes, extension methods, `where T : INumber<T>` (generic math), `[MaybeNullWhen]` | No LINQ! The goal is to write it by hand. |
| **Python** | `class Stack[T]`, `def f[T, U]`, `Protocol` as a constraint (`[N: Addable]`), dunder methods `__len__` / `__iter__`, `iter` / `next` | Types are checked only by a type checker (mypy, Pylance), the interpreter ignores them. No built-in `map` / `filter` / `sum` / `max`. |
| **Rust** | `struct Stack<T>`, `Option<T>` / `Option<&T>`, trait bounds (`T: Copy + Default + Add<Output = T>`), `impl Fn`, a newtype with its own `impl Add` | `#[derive(Default)]` on `Stack<T>` quietly requires `T: Default`, so write `Default` by hand. std's `max_by_key` returns the *last* element on ties. No iterator adapters, write the loops. |

**Compare:** Go `(T, bool)` vs. C# `bool TryX(out T)` vs. a Python exception (`IndexError`, same as `list.pop`) vs. Rust `Option<T>`. It's the same "what to return when there's nothing" problem in a different outfit. Rust needs no zero value: the value exists only inside `Some`.

---

## 05 · Queries over tunnel events

**Task:** Event counts per tunnel, the latest event, top N tunnels, a filter over a time window, an hourly histogram.

| | You practice | Trap |
|---|---|---|
| **Go** | maps of pointers (`map[string]*Stat`), `cmp.Or` for sorting by several keys, `time.Time`, arrays `[24]int` | `slices.SortFunc` sorts in place, so you sort the caller's data. Compare times with `Before` / `After` / `Equal`, not `==`. |
| **C#** | LINQ all the way: `GroupBy`, `Select`, `MaxBy`, `Max`, `Where`, `OrderBy`, `Take` | `MaxBy` returns a nullable. LINQ is lazy, materialize the result with `ToList()`. |
| **Python** | `Counter`, generator expressions, `sorted(key=attrgetter(...))`, `datetime` with a time zone, chained comparison `a <= x < b` | `list.sort()` sorts the caller's data. A `datetime` without a time zone can't be compared with one that has a zone (`TypeError`). `[[]] * 24` is the same list 24 times. |
| **Rust** | results that borrow from the input (`HashMap<&str, usize>`, `Vec<&Event>`), `entry(...).and_modify(...).or_insert(...)`, `or_insert_with`, `sort_by_key`, `SystemTime`, arrays `[usize; 24]` | std has no calendar (no dates, no time zones): the hour comes from Unix seconds, real projects use `chrono` or `jiff`. `or_insert(expensive())` evaluates its argument on every call, `or_insert_with(\|\| ...)` only for a new key. |

**Compare:** Go and Python have to be careful not to sort the caller's data. In Rust `&[Event]` makes that impossible: you can't sort through a shared reference, so you sort a new `Vec` of references.

---

## 06 · State machine: a tunnel lane

**Task:** Lane states (open, restricted, closed, maintenance), allowed transitions and a `Lane` object with history.

| | You practice | Trap |
|---|---|---|
| **Go** | nested `switch`, `String()` via an array, pointer receivers, unexported fields, variadic functions, a defensive copy (`slices.Clone`) | On an error in the middle of `Apply`, the state must stay as after the last valid action. |
| **C#** | switch on tuples `(state, action) switch`, `private set`, `params`, `IReadOnlyList` + `AsReadOnly()` | Returning a `List` as `IReadOnlyList` isn't enough, it can be cast back. |
| **Python** | `Enum` with the label as its value, `match (state, action)`, `@property` without a setter, `*args`, `tuple` as an immutable copy | `_state` is private by convention only. A default argument `def f(x=[])` is shared by all calls. |
| **Rust** | `match (state, action)` with `\|` patterns, `Display` on enums, an error struct, private fields, a slice `&[Action]` instead of variadics, returning `&[LaneState]` | A `Result` holds either a value or an error, so the original state (which Go returns next to the error) travels inside the error. Assign only after `?`, so a failed step leaves the state unchanged. |

**Compare:** Go copies the history, C# wraps it, Python converts it to a tuple. Rust just lends it out (`&[LaneState]`): no copy, read-only, and the borrow checker won't let `apply` run while someone still holds it.

---

## 07 · Client configuration: functional options vs. builder

**Task:** Build an HTTP client configuration with default values and optional settings with validation.

| | You practice | Trap |
|---|---|---|
| **Go** | **functional options** (`type Option func(*Client) error`), closures, `maps.Clone` | A struct copy shares the map with the original (shallow copy). |
| **C#** | **fluent builder**, `record` with `init` and `required`, object initializer, `with` expression | The builder must copy the headers in `Build()`, otherwise later calls also show up in already built options. |
| **Python** | **named arguments**: `KW_ONLY`, `frozen` dataclass, `field(default_factory=...)`, `__post_init__`, `dataclasses.replace`, `MappingProxyType` | `frozen` is shallow too: a dict inside can still be changed. A dataclass rejects a mutable default (`headers = {}`). |
| **Rust** | **consuming builder** (`fn timeout(mut self, ...) -> Self`), validation in `build()`, `derive(Clone)`, struct update syntax `..self.clone()`, `u32` so negative retries don't compile | Every setter moves the builder: to build two variants from one, `clone()` it first. The setters can't return a `Result` without breaking the chain, so everything is validated in `build()`. |

**Compare:** This is probably the biggest difference in "idioms" in the whole set. Builders are rarely used in Go. Python needs neither, because named arguments with default values are built into the language and `replace()` is the equivalent of C# `with`. Rust has no named or optional arguments either, so like C# it reaches for a builder. Its `Clone` is always deep for owned data, so Go's shared-map trap can't happen by accident.

---

## 08 · TTL cache and testable time

**Task:** A generic cache whose entries expire after a while. Time is injected so tests can advance it.

| | You practice | Trap |
|---|---|---|
| **Go** | a small `Clock` interface + a fake in the test, `sync.Mutex`, a generic struct | `sync.Mutex` is not reentrant: calling `Get` inside a locked method = deadlock. Don't call `load` under the lock. |
| **C#** | `TimeProvider` (built in since .NET 8), `Lock` + `lock`, `async`/`await`, `Task` | The compiler doesn't allow `await` inside `lock`. |
| **Python** | time as a plain function `Callable[[], float]`, a fake with `__call__`, `threading.Lock` + `with`, a sentinel `object()`, `__contains__` / `__len__` | `threading.Lock` isn't reentrant either (`RLock` is). A stored value `None` is not the same as "nothing in the cache". The GIL doesn't make compound operations atomic. |
| **Rust** | `Mutex<HashMap>` behind `&self` methods (interior mutability), a `Clock: Send + Sync` trait, `Arc<dyn Clock>`, `Instant`, a loader `impl FnOnce(&K) -> Result<V, E>` | std's `Mutex` isn't reentrant either. `get` returns a clone: a reference into the map can't outlive the lock guard. `lock().unwrap()` panics only if another thread panicked while holding the lock. |

**Compare:** Look at how the fake clock looks in the tests in each language. You'll need this pattern in every real project. In Rust the compiler won't let you touch the map without locking: the data is inside the `Mutex`.

---

## 09 · Iterators and lazy sequences

**Task:** Generators (`Range`, an infinite `Fibonacci`), lazy `Take` / `Filter` (`Where`), sliding windows and walking a map in sorted order.

| | You practice | Trap |
|---|---|---|
| **Go** | `iter.Seq` / `iter.Seq2`, range-over-func (Go 1.23+), `slices.Collect` | If `yield` returns `false`, you must stop immediately, otherwise the runtime panics. |
| **C#** | `yield return`, `yield break`, local functions | In a method with `yield`, even argument validation runs only on the first iteration, so split the method in two. |
| **Python** | generators (`yield`), `deque(maxlen=...)`, `enumerate`, infinite sequences. Then compare with `itertools`. | The same validation trap as in C#. Also: a generator can be iterated only once, a second `list(gen)` is empty. |
| **Rust** | `impl Iterator for` a struct (`next()`), generic adapters `Take<I>` / `Filter<I, P>`, `FnMut`, `by_ref()`, returning `impl Iterator`, windows as borrowed `&[T]` | Iterators do nothing until consumed: a `map(...)` without `collect`/`for` is just a warning. std's `step_by(0)` and `windows(0)` panic. No `yield` (yet): the state lives in struct fields. |

**Compare:** The `Laziness` / `Pipeline_IsLazy` / `test_laziness` / `laziness` test checks that the predicate is called only 6 times even though the source has a million elements. Rust and C#/Python iterators are pull-based (the caller asks for the next element), Go's are push-based (the iterator calls `yield`). That's why `break` in Rust needs no cooperation from the iterator.

---

## 10 · Concurrency: a worker pool and "first one wins"

**Task:** Process a list of jobs with limited parallelism (results in order, cancel the rest on error), and start several attempts at once and take the first successful one.

| | You practice | Trap |
|---|---|---|
| **Go** | goroutines, channels, `sync.WaitGroup`, `sync.Once`, `select`, `context.WithCancel`, `errors.Join` | Goroutine leak: an unread unbuffered channel. `select` picks randomly when several cases are ready. |
| **C#** | `Task`, `SemaphoreSlim`, `CancellationTokenSource`, `Task.WhenAll` / `WhenAny`, `ExceptionDispatchInfo` | `await Task.WhenAll` throws only one exception. Never `async void`. |
| **Python** | `asyncio`, `Semaphore`, `TaskGroup`, `asyncio.timeout`, `as_completed`, `task.cancel()`, `ExceptionGroup` / `except*` | `TaskGroup` always wraps errors in an `ExceptionGroup`. Never swallow `CancelledError`. A blocking `time.sleep` in a coroutine stops the whole event loop. |
| **Rust** | `std::thread::scope`, `mpsc` channels, `AtomicUsize` (`fetch_add`, `fetch_max`), `Mutex<Option<E>>`, `Send` / `Sync` bounds, a small cancellation token (given) | Keep a `Sender` alive and `for x in rx` waits forever: `drop` your own copy. A thread can't be killed: cancellation only works if the tasks check the token. |

**Compare:** Go's "communicate over channels" model vs. the C# and Python "await over tasks" model. Python has no tokens: cancellation is `task.cancel()`, which raises `CancelledError` at the next `await`. Run the Go tests with `-race`. Rust needs no race detector: sharing data between threads without `Sync` doesn't compile. Scoped threads all finish before `thread::scope` returns, so a leak like Go's leftover goroutines can't happen. The Rust version uses OS threads; async/await needs a runtime crate such as tokio.

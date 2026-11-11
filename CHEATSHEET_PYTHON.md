# Cheat sheet: Python for JS/TS developers

Target version: **Python 3.12** (as in `pyproject.toml`). Companion cheat sheets: [CHEATSHEET_GO.md](CHEATSHEET_GO.md), [CHEATSHEET_CSHARP.md](CHEATSHEET_CSHARP.md), [CHEATSHEET_RUST.md](CHEATSHEET_RUST.md).

Of these three languages, Python is the closest to JS: it's dynamic, has a GC, lambdas, generators and `async`/`await`. The main differences are **indentation as syntax**, **types that nobody enforces** and **errors instead of `undefined`**.

---

## 0. Three things that change everything

1. **A variable is a name that points to an object. Assignment never copies**, not even for `int`. What matters is whether the object is **mutable** (`list`, `dict`, `set`, your own classes) or **immutable** (`int`, `str`, `tuple`, `frozenset`). With mutable ones, a change shows up everywhere a name points to it, exactly like JS objects.
2. **The interpreter ignores type annotations.** `def f(x: int)` and `f("a")` runs just fine. Types are checked only by mypy or pyright (Pylance in VS Code). It's like TS that never gets compiled, only checked by the editor.
3. **Something missing is an error, not `undefined`.** `d["x"]` → `KeyError`, `obj.nope` → `AttributeError`, `xs[10]` → `IndexError`. Nothing silently returns nothing.

---

## 1. Truthiness and `None`

| value | JS | Python |
|---|---|---|
| `0`, `""`, `null` / `None` | falsy | falsy |
| `[]`, `{}` | **truthy** | **falsy** (an empty collection) |
| your own object | truthy | truthy, unless `__len__` / `__bool__` says otherwise |

```python
if not items: ...            # the idiomatic test for an empty list/dict/str
if x is None: ...            # None always with `is`, not ==
x or "default"               # ≈ JS x || "default" (replaces 0 and "" too!)
x if x is not None else d    # ≈ JS x ?? d (Python has neither ?? nor ?.)
d.get("k")                   # ≈ m.get(k): None instead of KeyError
getattr(obj, "a", None)      # ≈ obj?.a for an attribute
```

`is` compares identity (the same object), `==` compares value. Use `is` only for `None`, `True`/`False` and sentinels, never for numbers or strings.

---

## 2. Variables, types, basic syntax

| JS/TS | Python |
|---|---|
| `let x = 5` / `const x = 5` | `x = 5` (no `let`/`const`. A constant is just the `UPPER_CASE` convention) |
| `let x: number` | `x: int = 0` |
| `number` | `int` (**unlimited size**), `float`, `decimal.Decimal` (money) |
| `5 / 2 === 2.5` | `5 / 2 == 2.5`, integer division `5 // 2 == 2` |
| `Math.floor(-5 / 2)` | `-5 // 2 == -3` (rounds down, not toward zero like Go/C#) |
| `Math.round(2.5) === 3` | `round(2.5) == 2` (**banker's rounding** to even) |
| `a ? b : c` | `b if a else c` |
| `===` | `==` (no coercion: `1 == "1"` is `False`, but `1 == 1.0` is `True`) |
| `&&`, `\|\|`, `!` | `and`, `or`, `not` |
| `null`, `undefined` | `None` |
| `x++` | `x += 1` (`++` doesn't exist) |
| `0 < x && x < 10` | `0 < x < 10` (chained comparison) |
| `` `${a}-${b}` `` | `f"{a}-{b}"`, format `f"{x:.1f}"`, debug `f"{x=}"` → `x=5` |
| `const [a, b] = arr` | `a, b = arr`, `first, *rest = arr` |
| `[a, b] = [b, a]` | `a, b = b, a` |
| `for (let i = 0; i < n; i++)` | `for i in range(n):` |
| `for (const x of arr)` | `for x in arr:` |
| `arr.forEach((x, i) => …)` | `for i, x in enumerate(arr):` |
| `while (c)` | `while c:` |
| `switch` | `match` (section 7) |
| `try/catch/finally` | `try/except/finally` (+ `else`) |
| `using x = …` / dispose | `with open(path) as f:` (context manager) |
| `import { a } from "./m"` | `from package.module import a` |
| `export` | everything at module level can be imported, `_name` = private by convention only |
| `console.log(x)` | `print(x)`, `print(f"{x=}")`, `breakpoint()` (stops in the debugger) |

Block = a colon and indentation (4 spaces). An empty block = `pass`. An expression spanning several lines goes in parentheses, no `\` needed.

---

## 3. Strings

- `str` is a sequence of **Unicode code points**. Not UTF-16 like JS and C#, not bytes like Go.

```python
s = "kôň😀"
len(s)              # 4   (JS 5, C# 5, Go 9)
s[1]                # "ô"
s[-1]               # "😀"  (negative index from the end)
s.encode()          # bytes: b'k\xc3\xb4\xc5\x88\xf0\x9f\x98\x80', a separate type
```

- Immutable. Building up in a loop: `parts.append(…)` and finally `"".join(parts)`.
- Comparison and sorting go by code point (ordinal, like Go): `sorted(["z", "Ž", "a"]) == ["a", "z", "Ž"]`. It doesn't depend on the system culture.
- Number formatting doesn't depend on the culture either: `f"{1.5:.1f}"` is always `"1.5"` (unlike C#).
- Trap: the same text can come in different Unicode normalizations. `"kôň"` in NFD has 5 code points. Before comparing text from outside sources, use `unicodedata.normalize("NFC", s)`.

| JS | Python |
|---|---|
| `s.length` | `len(s)` |
| `s.split(",")` | `s.split(",")` |
| `s.split(/\s+/)` without empties | `s.split()` (no argument) |
| `arr.join(",")` | `",".join(arr)` (elements must be `str`: `",".join(map(str, nums))`) |
| `s.trim()` | `s.strip()` |
| `s.toLowerCase()` | `s.lower()` (for case-insensitive comparison: `s.casefold()`) |
| `s.includes(x)` | `x in s` |
| `s.startsWith(x)` | `s.startswith(x)` (also a tuple: `s.startswith(("a", "b"))`) |
| `s.indexOf(x)` | `s.find(x)` (→ -1) / `s.index(x)` (exception) |
| `s.replaceAll(a, b)` | `s.replace(a, b)` (replaces **all** occurrences) |
| `s.slice(1, -1)` | `s[1:-1]` |
| `s.repeat(3)` | `s * 3` |
| `s.padStart(5, "0")` | `s.rjust(5, "0")` / `f"{n:05d}"` |
| split at the first `=` (Go `strings.Cut`) | `key, sep, value = s.partition("=")` (`sep == ""` if `=` is missing) |
| `parseInt(s)` | `int(s)` (`ValueError`, `"12px"` won't pass like in JS) |
| `String(n)` | `str(n)` |
| `JSON.stringify` / `parse` | `json.dumps` / `json.loads` |

---

## 4. Collections

| JS/TS | Python | mutable? |
|---|---|---|
| `Array` | `list`: `[1, 2]` | yes |
| `readonly [a, b]` | `tuple`: `(1, 2)`, single-element **`(1,)`** | no |
| `Map` / object | `dict`: `{"a": 1}` | yes |
| `Set` | `set`: `{1, 2}`. Empty is `set()`, because **`{}` is an empty dict** | yes |
| frozen set | `frozenset` | no |

- A `dict` **remembers insertion order** (guaranteed). A Go map has random order, a C# `Dictionary` doesn't guarantee it.
- A dict key can be anything hashable: `str`, `int`, `tuple`, a `frozen` dataclass. Not a `list` or `dict`.
- `==` on collections compares **contents deeply**: `[1, [2]] == [1, [2]]` is `True`. JS compares references.

### list ≈ JS Array

| JS | Python |
|---|---|
| `arr.push(x)` | `xs.append(x)` |
| `arr.push(...ys)` | `xs.extend(ys)` |
| `arr.concat(ys)` | `xs + ys` |
| `arr.pop()` | `xs.pop()` (`IndexError` on empty) |
| `arr.shift()` | `xs.pop(0)` (O(n). For a queue use `collections.deque` and `popleft()`) |
| `arr.length` | `len(xs)` |
| `arr.includes(x)` | `x in xs` |
| `arr.indexOf(x)` | `xs.index(x)` (`ValueError` if missing) |
| `arr.at(-1)` | `xs[-1]` |
| `arr.slice(1, 3)` | `xs[1:3]` (a **copy**, not a view like a Go slice) |
| `[...arr]` | `xs.copy()` / `list(xs)` / `xs[:]` (shallow copy) |
| `structuredClone(arr)` | `copy.deepcopy(xs)` |
| `arr.reverse()` | `xs.reverse()` (in place) / `xs[::-1]` / `reversed(xs)` |
| `arr.sort()` | `xs.sort()` (in place, **returns `None`**) |
| `arr.toSorted()` | `sorted(xs)` (a new list) |
| `arr.sort((a, b) => …)` | `sorted(xs, key=lambda x: (-x.count, x.word))`, descending `reverse=True` |
| `[...new Set(arr)]` | `list(dict.fromkeys(xs))` (keeps order) / `set(xs)` |
| `Math.max(...arr)` | `max(xs)` (`ValueError` on empty, or `max(xs, default=0)`) |
| the element with the max value | `max(xs, key=f)` |
| `arr.reduce((a, b) => a + b)` | `sum(xs)` |
| `arr.some(f)` / `every(f)` | `any(f(x) for x in xs)` / `all(…)` |
| `arr.find(f)` | `next((x for x in xs if f(x)), None)` |
| `new Array(n).fill(0)` | `[0] * n` (careful: `[[]] * n` is **the same** list n times) |
| `Array.from({ length: n }, (_, i) => i * i)` | `[i * i for i in range(n)]` |
| `zip` (lodash) | `zip(a, b)`, strict `zip(a, b, strict=True)` |

Sorting uses `key` instead of a comparator: a tuple compares element by element, so `key=lambda x: (-x.count, x.word)` means "count descending, then word ascending". Sorting is stable.

### dict ≈ JS Map

| JS `Map` | Python `dict` |
|---|---|
| `new Map()` | `{}` / `dict()` |
| `m.get(k)` → `undefined` | `d[k]` → **`KeyError`** / `d.get(k)` → `None` / `d.get(k, 0)` |
| `m.set(k, v)` | `d[k] = v` |
| `m.has(k)` | `k in d` |
| `m.delete(k)` | `del d[k]` / `d.pop(k, None)` |
| `m.size` | `len(d)` |
| `for (const [k, v] of m)` | `for k, v in d.items():` |
| `[...m.keys()]` | `list(d)` |
| `{ ...a, ...b }` | `{**a, **b}` / `a \| b` |
| `Object.fromEntries(pairs)` | `dict(pairs)` / `{k: v for k, v in pairs}` |

Idioms:

```python
counts[w] = counts.get(w, 0) + 1         # JS: m.set(w, (m.get(w) ?? 0) + 1)
counts = Counter(words)                   # even better (collections.Counter)

groups.setdefault(k, []).append(v)        # JS: (groups[k] ??= []).push(v)
groups = defaultdict(list)                # or a defaultdict:
groups[k].append(v)                       # a missing key is created by itself
```

`defaultdict` trap: even a plain **read** `groups["x"]` inserts the key. Return a plain `dict` to the outside world.

---

## 5. Comprehensions ≈ map / filter

```python
[x * 2 for x in xs]                   # xs.map(x => x * 2)
[x for x in xs if x > 0]              # xs.filter(x => x > 0)
[x * 2 for x in xs if x > 0]          # filter + map at once
{w: len(w) for w in words}            # Object.fromEntries(words.map(w => [w, w.length]))
{len(w) for w in words}               # new Set(words.map(w => w.length))
[y for row in grid for y in row]      # grid.flat() (the for order is like nested loops)
sum(x * x for x in xs)                # generator expression: lazy, no intermediate list
```

- `[…]` builds a list right away. `(…)` is a **generator**: lazy like LINQ, but it can be iterated **only once**. A second pass is empty.
- The built-in `map()` and `filter()` exist and are lazy, but a comprehension is more readable and more idiomatic.

---

## 6. Classes, dataclasses, dunder methods

```python
class Lane:
    count = 0                                  # CLASS attribute: shared by all instances

    def __init__(self, name: str) -> None:     # constructor. self is explicit (≈ this)
        self.name = name                       # instance attributes come into being on assignment
        self._state = State.OPEN               # _ = "private", but only by convention

    @property
    def state(self) -> State:                  # ≈ get state(). Without a setter it can't be assigned
        return self._state

    def apply(self, *actions: Action) -> None: # ≈ apply(...actions)
        ...

    @classmethod
    def from_json(cls, data: dict) -> "Lane":  # an alternative constructor
        return cls(data["name"])

    def __repr__(self) -> str:                 # representation for debugging (REPL, debugger)
        return f"Lane({self.name!r})"
```

Trap: a mutable value as a class attribute (`items = []`) is **one for all instances**. State belongs in `__init__`.

### dataclass ≈ C# record

```python
from dataclasses import dataclass, field, replace

@dataclass(frozen=True)                   # frozen = immutable (shallowly!) and hashable
class Point:
    x: int
    y: int = 0

p = Point(1, 2)                           # constructor, __repr__, __eq__ are generated
p == Point(1, 2)                          # True: by value
replace(p, y=5)                           # ≈ JS { ...p, y: 5 } / C# p with { Y = 5 }

@dataclass
class Bag:
    items: list[str] = field(default_factory=list)   # NOT `= []`
```

### Dunder methods = the language's "protocols"

| method | what it enables |
|---|---|
| `__init__` | constructor |
| `__repr__` / `__str__` | output for debugging / for humans (`print`, `f"{x}"`) |
| `__eq__`, `__hash__` | `==`, use as a dict key |
| `__lt__` | `<`, `sorted()`, `max()` |
| `__len__`, `__bool__` | `len(x)`, `if x:` |
| `__iter__` | `for v in x` |
| `__contains__` | `v in x` |
| `__getitem__` | `x[k]` |
| `__call__` | `x()` (the object behaves like a function) |
| `__enter__` / `__exit__` | `with x:` |

- Inheritance: `class B(A):`, `super().__init__(…)`. Multiple inheritance too. In practice, prefer composition.
- A nominal interface: `abc.ABC` + `@abstractmethod`. A structural one: `Protocol` (section 7).

---

## 7. Type annotations, Protocol, Enum, match

| TS | Python |
|---|---|
| `number[]` | `list[int]` |
| `Record<string, number>` | `dict[str, int]` |
| `[string, number]` | `tuple[str, int]` |
| `string \| null` | `str \| None` |
| `unknown` / `any` | `object` / `Any` |
| `(x: number) => string` | `Callable[[int], str]` |
| `Iterable<T>` | `Iterable[T]` (from `collections.abc`) |
| `"a" \| "b"` | `Literal["a", "b"]` |
| `type Id = string` | `type Id = str` (3.12) |
| `function f<T>(x: T): T` | `def f[T](x: T) -> T:` (3.12) |
| `class Stack<T>` | `class Stack[T]:` (3.12) |
| `T extends X` | `def f[T: X](…)` |
| interface (structural) | `typing.Protocol` |

```python
class Sensor(Protocol):            # like a Go interface / TS interface: the shape is enough
    sensor_id: str
    def evaluate(self) -> Severity: ...

class CoSensor:                    # inherits nothing, and is still a Sensor (for the type checker)
    ...
```

Nobody checks a Protocol at runtime. `@runtime_checkable` allows `isinstance(x, Sensor)`, but only verifies that the members exist.

### Enum

```python
class Severity(IntEnum):     # IntEnum behaves like an int: <, max(), sorted work
    OK = 0
    WARNING = 1

class State(Enum):           # a plain Enum: the value can be anything
    OPEN = "open"

State.OPEN.name              # "OPEN"
State.OPEN.value             # "open"
State("open")                # lookup by value (ValueError if it doesn't exist)
State["OPEN"]                # by name
```

### match (3.10+) ≈ C# switch with patterns

```python
match sensor:
    case CoSensor(ppm=p) if p >= 100:          # type + attribute + guard
        ...
    case CoSensor(sid, ppm):                   # positional (a dataclass has __match_args__)
        ...
    case {"type": "smoke", **rest}:            # dict
        ...
    case [first, *rest]:                       # sequence
        ...
    case Severity.OK | Severity.WARNING:       # constants WITH A DOT, | = or
        ...
    case _:                                    # default
        ...

match (state, action):                         # a tuple ≈ C# (state, action) switch
    case (State.OPEN, Action.CLOSE):
        ...
```

**Trap:** a bare name in a `case` is a **capture into a variable**, not a comparison. `case OK:` matches absolutely everything. Always write constants with a dot (`Severity.OK`).

---

## 8. Exceptions

```python
class ConfigError(Exception):
    def __init__(self, line: int, msg: str) -> None:
        super().__init__(f"line {line}: {msg}")
        self.line = line

try:
    value = int(raw)
except (KeyError, ValueError) as e:            # several types at once
    raise ConfigError(3, "bad value") from e   # ≈ new Error(msg, { cause: e }) → e.__cause__
except ConfigError:
    raise                                      # rethrows with the traceback preserved
else:
    ...                                        # runs only if nothing was raised
finally:
    ...
```

- **EAFP**: Python prefers "try and catch" (`try: d[k] except KeyError`) over "check first" (`if k in d`).
- `raise X from None` hides the original exception from the output.
- A bare `except:` also catches Ctrl+C (`KeyboardInterrupt`) and `sys.exit()`. At least `except Exception:`, ideally a specific type.
- Don't name your own class after a built-in one (`SyntaxError`, `TimeoutError`, …).
- `ExceptionGroup` + `except*` (3.11) ≈ `AggregateError`.
- `assert` is not for input validation: `python -O` turns it off.

---

## 9. Functions

```python
def connect(url: str, /, timeout: float = 30, *, retries: int = 3, **extra: str) -> Conn:
    ...
#     before / positional only ─┘                └─ after * by name only

connect("https://x", retries=5)        # a named argument
connect(*args, **options)              # ≈ connect(...args) + unpacking an object
```

- `*args` ≈ `...rest` (a tuple), `**kwargs` ≈ the rest of an object (a dict).
- `lambda x: x * 2` is a single expression only. For anything longer, write a `def`.
- **Trap: the default value is evaluated once, at definition time.** `def f(x=[])` shares the list between calls, `def log(t=datetime.now())` keeps the same time forever. Use `None` and create the value inside the function.
- You can rebind a variable from the enclosing function only with `nonlocal x` (from the module with `global x`). Without it, `x += 1` creates a local variable and fails.
- **Trap: closures in a loop** behave like JS `var`, not `let`: `[lambda: i for i in range(3)]` returns `2` three times. Fix: `lambda i=i: i`.
- A decorator (`@functools.cache`, `@property`, `@dataclass`) is a function that wraps a function or a class.
- Useful: `functools.partial`, `operator.attrgetter("at")`, `operator.itemgetter(1)`.

---

## 10. Iterators and generators ≈ JS generators

| JS | Python |
|---|---|
| `function* f()` | `def f():` with `yield` (every function with `yield` is a generator) |
| `yield* other` | `yield from other` |
| `for (const x of gen)` | `for x in gen:` |
| `[...gen]` | `list(gen)` |
| `gen.next()` | `next(gen)` (`StopIteration` at the end) / `next(gen, default)` |
| `async function*` + `for await` | `async def` with `yield` + `async for` |

`itertools` (≈ lodash for iterators):

| function | what it does |
|---|---|
| `islice(it, n)` | the first n (≈ take) |
| `count(start)` | an infinite counter |
| `chain(a, b)` | one after the other |
| `pairwise(it)` | `(a, b), (b, c), …` |
| `batched(it, n)` | in chunks of n (3.12, ≈ C# `Chunk`) |
| `groupby(it, key)` | groups **only adjacent** equal elements, so sort first |
| `accumulate(it)` | running totals |
| `product(a, b)` | Cartesian product (nested loops) |

- A generator can be iterated **only once**. `range(n)` is lazy but repeatable (it's not a generator).
- The generator body, including argument validation, runs only on the first `next()`. The same trap as in C#. The fix is a wrapper without `yield` that validates and returns the inner generator.

---

## 11. Concurrency

### asyncio ≈ the JS event loop

| JS | Python |
|---|---|
| one thread + event loop | the same: `asyncio` |
| `async function f()` | `async def f():` |
| calling it returns a Promise **that is already running** | calling it returns a coroutine **that doesn't run** until you await it or start it with `asyncio.create_task` |
| `await p` | `await coro` |
| `Promise.all` | `asyncio.gather(*aws)` / `async with asyncio.TaskGroup() as tg:` |
| `Promise.race` / `any` | `asyncio.as_completed(…)` / `asyncio.wait(…, return_when=FIRST_COMPLETED)` |
| `AbortController` | `task.cancel()` → `CancelledError` at the next `await` (no tokens) |
| `await sleep(ms)` | `await asyncio.sleep(s)` (**seconds**) |
| timeout | `async with asyncio.timeout(2):` |
| top-level await | `asyncio.run(main())` |

- Blocking code (`time.sleep`, `requests.get`, a heavy computation) in a coroutine stops the **whole** event loop. For such calls: `await asyncio.to_thread(f, …)`.
- Don't catch and swallow `CancelledError`. If you catch it, pass it on with `raise`.
- `TaskGroup` always wraps errors in an `ExceptionGroup`.

### Threads and processes

```python
with ThreadPoolExecutor(max_workers=8) as pool:      # concurrent.futures
    results = list(pool.map(fetch, urls))

lock = threading.Lock()
with lock:                                           # ≈ lock {} / mu.Lock(); defer mu.Unlock()
    ...
```

- **GIL**: in regular CPython, Python code runs in only one thread at a time. Threads help with I/O, not with computation. For CPU work use `ProcessPoolExecutor`. (Since 3.13 there is an experimental build without the GIL.)
- The GIL doesn't make compound operations atomic: `counter += 1` from several threads needs a lock.
- `threading.Lock` is not reentrant (a second `acquire` in the same thread = deadlock). `RLock` is reentrant.

---

## 12. Project and tools

| JS | Python |
|---|---|
| `package.json` | `pyproject.toml` |
| `node_modules` | `.venv` (`python -m venv .venv` + activation) |
| `npm install x` | `pip install x` / `uv add x` |
| `npx jest` | `python -m pytest` |
| ESLint + Prettier | `ruff check` + `ruff format` |
| `tsc --noEmit` | `mypy` / `pyright` |
| `if (require.main === module)` | `if __name__ == "__main__":` |

---

## 13. Common traps (summary)

- `list.sort()` sorts in place and returns `None`. `sorted()` returns a new list.
- A default argument `def f(x=[])` is evaluated once and shared.
- A class attribute `items = []` is shared by all instances.
- `[[]] * n` is the same list n times.
- `{}` is an empty dict, not a set.
- `(1)` is a number, a tuple is `(1,)`.
- `[]` and `{}` are falsy (truthy in JS). `x or d` also replaces `0` and `""`.
- `d[k]` on a missing key raises `KeyError`. `defaultdict` inserts the key on read.
- `case OK:` in `match` matches everything, write constants with a dot.
- Closures in a loop bind the variable, not the value (`lambda i=i: i`).
- A generator can be iterated only once.
- `-5 // 2 == -3`, `round(2.5) == 2`, `0.1 + 0.2 != 0.3`.
- Types aren't checked at runtime.
- `async def` doesn't run until you await it. `time.sleep` in a coroutine blocks everything.
- `threading.Lock` is not reentrant. The GIL doesn't make `+=` atomic.

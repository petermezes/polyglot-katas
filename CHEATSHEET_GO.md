# Cheat sheet: Go for JS/TS developers

Target version: **Go 1.23** (as in `go.mod`). Companion cheat sheets: [CHEATSHEET_CSHARP.md](CHEATSHEET_CSHARP.md), [CHEATSHEET_PYTHON.md](CHEATSHEET_PYTHON.md), [CHEATSHEET_RUST.md](CHEATSHEET_RUST.md).

---

## 0. Three things that change everything

1. **Everything is a value and gets copied on assignment.** Structs and arrays too. In JS objects are passed by reference, in Go they aren't. If you want to share, use a pointer `*T`.
   The exception that "breaks" this: slices, maps, channels and funcs contain a pointer inside, so a copy shares the data (see below).
2. **Every type has a zero value.** There is no `undefined`. A variable has a value right from its declaration.
3. **Errors are return values, not exceptions.** `v, err := f()` and immediately `if err != nil`.

---

## 1. Zero values

| type | zero value | can you use it right away? |
|---|---|---|
| `int`, `float64` | `0` | yes |
| `string` | `""` | yes |
| `bool` | `false` | yes |
| struct | all fields at their zero value | yes (good Go code is designed for it) |
| `[]T` (slice) | `nil` | yes: `len` = 0, `range` and `append` work |
| `map[K]V` | `nil` | **read only**. Writing = panic, hence `make` |
| `*T`, interface, func, chan | `nil` | no: dereferencing/calling = panic |

```go
var s []string        // nil slice, but behaves like []
s = append(s, "a")    // OK

var m map[string]int  // nil map
_ = m["x"]            // OK → 0
m["x"] = 1            // panic: assignment to entry in nil map
m = make(map[string]int)  // or map[string]int{}
```

Zero value of a generic type: `var zero T`.

### The zero value is free

Zero value = **memory filled with zeros**. `var x T` reserves `sizeof(T)` bytes and zeroes them. No code runs (Go has no constructors) and nothing is allocated.

```go
type Index struct { ByName, ByCity, ByZip, ByTag, ByOwner map[string]int }
var x Index                 // 40 zero bytes = 5× nil pointer, no table
make([]Index, 10_000)       // one zeroing of 400 KB, not 50,000 allocations
```

That's why a nil map is "unfinished". If it were an empty table right away, every declaration would silently allocate. Whatever costs performance should be visible in the code (`make`).

Two patterns for living with it:

```go
func NewIndex() *Index { return &Index{ByName: map[string]int{}} }   // a) constructor (just a convention)

func (i *Index) Add(name string, id int) {                           // b) lazy initialization:
    if i.ByName == nil { i.ByName = make(map[string]int) }          //    a zero-value Index works
    i.ByName[name] = id                                              //    even without a constructor
}
```

---

## 2. Variables, types, basic syntax

| JS/TS | Go |
|---|---|
| `let x = 5` | `x := 5` (inside functions only) / `var x = 5` |
| `let x: number` | `var x int` (= 0) |
| `const X = 5` | `const X = 5` (only for numbers, strings, bools, not for objects) |
| `number` | `int`, `int64`, `float64`, … (no implicit conversions: `float64(i)`) |
| `5 / 2 === 2.5` | `5 / 2 == 2` (**integer division** for int) |
| `a ? b : c` | doesn't exist. Write an `if` |
| `===` | `==` (types must match, otherwise it won't compile) |
| `export function Foo` | uppercase = exported: `func Foo`, lowercase = private to the package |
| `for (let i = 0; i < n; i++)` | `for i := 0; i < n; i++` or `for i := range n` |
| `while (c)` | `for c { }` |
| `for (const x of arr)` | `for _, x := range arr` |
| `arr.forEach((x, i) => …)` | `for i, x := range arr` |
| `try { } finally { }` | `defer cleanup()` (runs when the function returns) |

`_` = "I'm throwing this value away". An unused variable or import is a **compile error**, not a warning.

---

## 3. Strings, bytes, runes

- `string` = an immutable sequence of **bytes** (UTF-8). A JS string is a sequence of UTF-16 units.
- `byte` = `uint8` = one byte.
- `rune` = `int32` = one Unicode code point (character).

```go
s := "kôň😀"
len(s)                     // 9  bytes (!)
utf8.RuneCountInString(s)  // 4  characters  ≈ JS [...s].length
s[1]                       // 195 (a byte, not "ô")
for i, r := range s { }    // i = byte offset (0,1,3,5), r = rune
[]rune(s)                  // ≈ JS [...s]
```

| JS | Go |
|---|---|
| `s.length` | `len(s)` (bytes!) / `utf8.RuneCountInString(s)` |
| `s.split(",")` | `strings.Split(s, ",")` |
| `s.split(/\s+/)` | `strings.Fields(s)` |
| `arr.join(",")` | `strings.Join(arr, ",")` |
| `s.trim()` | `strings.TrimSpace(s)` |
| `s.toLowerCase()` | `strings.ToLower(s)` |
| `s.includes(x)` | `strings.Contains(s, x)` |
| `s.startsWith(x)` | `strings.HasPrefix(s, x)` |
| `` `${a}-${b}` `` | `fmt.Sprintf("%s-%d", a, b)` |
| `parseInt(s)` | `n, err := strconv.Atoi(s)` |
| `String(n)` | `strconv.Itoa(n)` |
| building up with `+=` in a loop | `var b strings.Builder; b.WriteString(…)` |

---

## 4. Array vs slice

| | Go array `[3]int` | Go slice `[]int` | JS `Array` |
|---|---|---|---|
| length | fixed, part of the type | dynamic | dynamic |
| assignment | copies the whole array | copies the header, **shares the data** | shares the reference |
| typical use | rarely (`[24]int` histogram) | **all the time** | all the time |

A slice is a small struct (header): `{ptr to the backing array, len, cap}`.

```go
s := []int{1, 2, 3}        // literal
s := make([]int, 0, 100)   // len 0, cap 100 (preallocation)
s = append(s, 4)           // ALWAYS assign back: append returns a new header
s[1:3]                     // sub-slice: shares the backing array (≈ subarray, NOT JS slice)
c := slices.Clone(s)       // a real copy (≈ JS [...arr])
```

### Trap: shared backing array

```go
a := make([]int, 3, 10)
b := append(a, 1)   // writes into a's array at index 3
c := append(a, 2)   // overwrites the same index 3
// b == [0 0 0 2]  (!), c == [0 0 0 2]
```

Go is **not immutable like Immer**. "Assign back" exists because `len` is a value in the header, not because of immutability. When you return an internal slice or store someone else's, do `slices.Clone`.

### JS Array methods → Go

| JS | Go |
|---|---|
| `arr.push(x)` | `arr = append(arr, x)` |
| `arr.length` | `len(arr)` |
| `arr.includes(x)` | `slices.Contains(arr, x)` |
| `arr.indexOf(x)` | `slices.Index(arr, x)` |
| `arr.findIndex(f)` | `slices.IndexFunc(arr, f)` |
| `arr.sort()` | `slices.Sort(arr)` (in place, like JS) |
| `arr.sort((a,b) => …)` | `slices.SortFunc(arr, func(a, b T) int { return cmp.Compare(a.X, b.X) })` |
| sorting by several keys | `cmp.Or(cmp.Compare(b.N, a.N), cmp.Compare(a.W, b.W))` |
| `arr.toSorted()` | `s := slices.Clone(arr); slices.Sort(s)` |
| `arr.reverse()` | `slices.Reverse(arr)` |
| `[...new Set(arr)]` | `slices.Sort(arr); arr = slices.Compact(arr)` (Compact removes only **adjacent** duplicates) |
| `Math.max(...arr)` | `slices.Max(arr)` (panics on empty) |
| `Math.max(a, b)` | `max(a, b)` (builtin) |
| `arr.map / filter / reduce` | **a loop** (not in the standard library, on purpose) |
| `arr1 === arr2` element-wise | `slices.Equal(a, b)` |

---

## 5. Maps

| JS `Map` | Go `map[K]V` |
|---|---|
| `new Map()` | `make(map[K]V)` / `map[K]V{}` |
| `m.set(k, v)` | `m[k] = v` |
| `m.get(k)` → `undefined` | `m[k]` → **zero value** |
| `m.has(k)` | `v, ok := m[k]` ("comma ok") |
| `m.delete(k)` | `delete(m, k)` |
| `m.size` | `len(m)` |
| `for (const [k, v] of m)` | `for k, v := range m` (**random order**) |
| `[...m.keys()].sort()` | `slices.Sorted(maps.Keys(m))` |
| `new Map(m)` | `maps.Clone(m)` |

### Idioms thanks to zero values

```go
counts[w]++                            // JS: m.set(w, (m.get(w) ?? 0) + 1)
groups[k] = append(groups[k], v)       // JS: (groups[k] ??= []).push(v)
seen := map[string]bool{}; seen[x] = true; if seen[y] { }   // set
```

### Why writing to a nil map panics

A map is a pointer to a hash table in the runtime. If writing to a nil map silently created a table, then in `fill(m)` the new table would be assigned only to the local copy inside `fill` and the caller would still have `nil`. The data would be lost without an error. With slices this can't happen, because `append` returns a new value that you have to assign.

### Why the order is random

A hash table has no natural order. Go deliberately starts each `range` at a random position, so a dependency on order shows up right away in a test and not only after a Go upgrade. If you need an order, sort the keys.

### Map of structs vs map of pointers

```go
m := map[string]Stat{}
m["a"].Count++         // does NOT compile: a value in a map is not addressable
m2 := map[string]*Stat{}
m2["a"] = &Stat{}; m2["a"].Count++   // OK
```

---

## 6. Structs, methods, pointers (instead of classes)

```go
type Lane struct {
    ID      string     // exported field
    history []State    // private field (lowercase)
}

func NewLane(id string) *Lane { return &Lane{ID: id} }   // a "constructor" is just a convention

func (l Lane) Name() string { return l.ID }               // value receiver: gets a COPY
func (l *Lane) Apply(s State) { l.history = append(l.history, s) }   // pointer receiver: modifies the original
```

- If a method changes state, use a **pointer receiver** `*T`. For consistency, then use it on all methods of that type.
- `&x` = address, `*p` = the value at the address. `p.Field` works through a pointer too (automatic dereference).
- There is no inheritance, there is **embedding** (composition): `type B struct { A }` and the methods of `A` are available on `B`.
- A struct copy is **shallow**: maps and slices inside are shared with the original (`maps.Clone`, `slices.Clone`).

### Pointers

Go **always** passes by value (a pointer is a value too, a copy of the address). In JS the type decides about sharing (objects are shared, primitives copied). In Go you decide: without `*` it's copied, with `*` it's shared.

```go
a := Counter{N: 1}
b := a        // a copy of the whole struct
b.N = 2       // a.N == 1
pa := &a      // address of a
pa.N = 3      // a.N == 3
```

| notation | meaning |
|---|---|
| `*Counter` | the type "pointer to Counter" |
| `&a` | the address of variable `a` |
| `*pa` | the value at the address |
| `pa.N` | automatic dereference (no need for `(*pa).N`) |
| `&Counter{N: 1}` | create and return a pointer right away (the most common form) |

Compared to C, Go is safer:
- There is no pointer arithmetic.
- Memory is managed by the GC, so no `free` and no dangling pointers.
- `return &local` is fine. The compiler moves the variable to the heap (escape analysis, `go build -gcflags=-m` prints `moved to heap`).
- Dereferencing `nil` panics.

### What gets copied when passing to a function

Some types have a pointer inside. Only what is in their own bytes is copied:

| you pass | size | what gets copied | does the caller see changes made in the function? |
|---|---|---|---|
| struct | sum of the fields | all fields | no |
| `*T` | 8 B | the address | yes |
| map | 8 B | a pointer to the table | **yes** (the same table) |
| slice | 24 B | header: ptr, len, cap | `s[i] = x` yes, **`append` no** (only changes the local `len`) |
| string | 16 B | ptr + len | a string is immutable, so it doesn't matter |

This also explains the nil map panic: the function gets a copy of the pointer, so it can write into an existing table, but it can't hand a new table back to the caller.

### When to use a pointer

- The method changes state → pointer receiver `*T`. Then all methods of the type for consistency.
- The struct contains a `sync.Mutex` (a copy of a lock is a different lock) or is large → `*T`.
- Small immutable values (a point, `time.Time`, an enum) → a value.
- `*[]T` and `*map[K]V` almost never. If a function should change a slice's length, have it return the slice (like `append`).

### Equality

| | Go | JS |
|---|---|---|
| struct `==` | compares fields (if they are comparable) | objects: by reference |
| array `==` | element-wise | by reference |
| slice, map `==` | not possible (only `== nil`). Use `slices.Equal`, `maps.Equal` | by reference |

---

## 7. Interfaces

```go
type Sensor interface { Severity() Level }

type COSensor struct{ PPM float64 }
func (c COSensor) Severity() Level { … }   // NO "implements": having the method is enough

var _ Sensor = COSensor{}   // compile-time check that the type satisfies the interface
```

- Implicit, like structural types in TS: whoever has the methods satisfies the interface.
- Pointer receiver: only `*T` satisfies the interface, not `T`.
- `any` = `interface{}` ≈ TS `unknown`.
- A type switch ≈ type narrowing in TS:

```go
switch v := s.(type) {
case COSensor:   fmt.Println(v.PPM)
case *Smoke:     …
default:         …
}
c, ok := s.(COSensor)   // type assertion with comma-ok
```

- An enum via `iota`:

```go
type Level int
const (
    Ok Level = iota   // 0
    Warn              // 1
    Alarm             // 2
)
func (l Level) String() string { return [...]string{"OK", "WARN", "ALARM"}[l] }   // fmt.Stringer
```

---

## 8. Errors (instead of try/catch)

```go
var ErrNotFound = errors.New("not found")            // sentinel ≈ an error constant

type SyntaxError struct{ Line int }                 // a custom type
func (e *SyntaxError) Error() string { return fmt.Sprintf("line %d", e.Line) }

func parse() (Config, error) {
    v, err := read()
    if err != nil {
        return Config{}, fmt.Errorf("parse: %w", err)   // %w wraps ≈ new Error(msg, { cause })
    }
    return v, nil
}

errors.Is(err, ErrNotFound)          // ≈ walks the cause chain and compares
var se *SyntaxError
if errors.As(err, &se) { se.Line }   // ≈ catch (e) { if (e instanceof SyntaxError) }
errors.Join(e1, e2)                  // ≈ AggregateError
```

`panic` is for programmer errors (index out of range, nil map), not for normal control flow. `recover` in a `defer` catches it, but don't use it in regular code.

---

## 9. Functions, closures, generics

```go
func divmod(a, b int) (int, int) { return a / b, a % b }   // multiple return values
q, r := divmod(7, 2)

func sum(nums ...int) int { }       // variadic ≈ (...nums)
sum(xs...)                          // spread ≈ sum(...xs)

add := func(a, b int) int { return a + b }   // closure ≈ arrow function

type Option func(*Client) error     // a function type
```

Since Go 1.22 each loop iteration has its own variable, so closures in a loop work like `let` in JS.

```go
func Map[T, U any](xs []T, f func(T) U) []U { … }
func Max[T cmp.Ordered](a, b T) T { … }           // constraint ≈ TS extends
type Number interface { ~int | ~int64 | ~float64 }  // ~ = also types built on int
type Stack[T any] struct { items []T }
```

`comparable` = you can use `==` on it (and therefore use it as a map key).

---

## 10. Iterators (Go 1.23) ≈ JS generators

```go
func Range(n int) iter.Seq[int] {          // ≈ function* range(n)
    return func(yield func(int) bool) {
        for i := range n {
            if !yield(i) { return }        // false = the consumer is done (break). You MUST stop
        }
    }
}
for x := range Range(5) { }                // ≈ for (const x of range(5))
slices.Collect(seq)                        // ≈ [...gen]
maps.Keys(m), slices.Values(s)             // built-in iterators
```

---

## 11. Concurrency

| JS | Go |
|---|---|
| one thread + event loop | real parallelism, goroutines run on multiple cores |
| `async function` / Promise | `go f()` (starts a goroutine, no return value) |
| `await p` | read from a channel `<-ch` or `wg.Wait()` |
| `Promise.all` | `sync.WaitGroup` + results into a slice by index |
| `Promise.race` | `select` over several channels |
| `AbortController` / `signal` | `context.WithCancel` / `ctx.Done()` |
| `setTimeout` | `time.After(d)`, `time.AfterFunc` |
| nothing (not needed) | `sync.Mutex`: state shared between goroutines must be locked |

```go
ch := make(chan int)        // unbuffered: the sender waits for the receiver
ch := make(chan int, 10)    // buffered
ch <- 1                     // send
v, ok := <-ch               // receive (ok == false after close)
close(ch)
for v := range ch { }       // reads until it's closed

select {
case v := <-results:  …
case <-ctx.Done():    return ctx.Err()
case <-time.After(time.Second): …
}

var mu sync.Mutex
mu.Lock(); defer mu.Unlock()   // NOT reentrant: a second Lock in the same goroutine = deadlock
```

Run tests with `go test -race`.

---

## 12. Common traps (summary)

- `len(string)` is bytes, not characters.
- Writing to a nil map = panic.
- A forgotten `x = append(x, …)` = lost data.
- `append` and sub-slices share the backing array, hence `slices.Clone`.
- `slices.Sort` sorts the caller's data (in place).
- `range` order over a map is random.
- Integer division `5/2 == 2`.
- Pointer vs value receiver: a method with a value receiver modifies a copy.
- Compare times with `t.Before/After/Equal`, not `==`.
- An unused variable/import = compile error.
- An unread unbuffered channel = the goroutine hangs forever (leak).

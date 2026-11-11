# Cheat sheet: C# for JS/TS developers

Target version: **C# 14 / .NET 10** (as in the `.csproj`). Companion cheat sheets: [CHEATSHEET_GO.md](CHEATSHEET_GO.md), [CHEATSHEET_PYTHON.md](CHEATSHEET_PYTHON.md), [CHEATSHEET_RUST.md](CHEATSHEET_RUST.md).

C# is closer to JS/TS than Go: it has classes, exceptions, lambdas, `async`/`await`, generators, `?.` and `??`. The main differences are **nominal types**, **value vs reference types** and **culture** when working with text.

---

## 0. Three things that change everything

1. **A `class` is a reference (like a JS object), a `struct` is a value (it gets copied).** `int`, `double`, `bool`, `DateTime`, `decimal` are structs.
2. **Types are nominal, not structural.** You must declare an interface explicitly (`class X : ISensor`), otherwise the type doesn't satisfy it, even if it has the right methods. In TS and Go the shape is enough.
3. **Strings and numbers depend on the culture** (`CultureInfo.CurrentCulture`). Sorting, `ToLower`, number formatting. See section 3.

---

## 1. Variables, types, basic syntax

| JS/TS | C# |
|---|---|
| `let x = 5` | `var x = 5;` (the type is inferred, it's still static) |
| `let x: number` | `int x;` (a local variable must be assigned before use) |
| `const X = 5` | `const int X = 5;` (compile-time constants only) / a `readonly` field |
| `number` | `int`, `long`, `double`, `decimal` (money) |
| `5 / 2 === 2.5` | `5 / 2 == 2` (**integer division** for int) |
| `a ? b : c` | the same |
| `x ?? y`, `x?.y`, `x ??= y` | the same |
| `x!` (TS non-null) | `x!` (null-forgiving, also just for the compiler) |
| `string \| null` | `string?` (with `<Nullable>enable</Nullable>`) |
| `number \| null` | `int?` (= `Nullable<int>`, a value type can't be null otherwise) |
| `` `${a}-${b}` `` | `$"{a}-{b}"`, format `$"{x:F1}"` |
| `export` | `public`. The default is `private` (members) / `internal` (types) |
| `for (const x of arr)` | `foreach (var x in arr)` |
| `try/catch/finally` | the same + `using var x = …;` (≈ automatic `Dispose`, like TS `using`) |

**Zero values** exist here too: `default(T)` is `0`, `false`, `null`… Class fields get them automatically, local variables don't.

---

## 2. Value vs reference types, equality

| | `class` | `struct` | `record` (class) | `record struct` |
|---|---|---|---|---|
| assignment | shares the reference | copies | shares the reference | copies |
| `==` | by reference (like JS) | must be defined | **by value** | by value |
| null | yes | no (only `T?`) | yes | no |

```csharp
record P(int X, int Y);                  // positional record: constructor, properties, ==, ToString
var a = new P(1, 2);
a == new P(1, 2)                         // true
var b = a with { Y = 5 };                // ≈ JS { ...a, y: 5 }
```

---

## 3. Strings and culture

- `string` is UTF-16, **just like in JS**. `"kôň😀".Length == 5` (an emoji is 2 units).
- Number of code points: `s.EnumerateRunes().Count()` (≈ JS `[...s].length`). Graphemes: `new StringInfo(s).LengthInTextElements`.
- `==` on strings compares **ordinally** (byte by byte). That's OK.

### Trap: culture

```csharp
// CurrentCulture = sk-SK
new[] { "ch", "d", "c", "h" }.OrderBy(w => w)                          // c d h ch  (!)
new[] { "ch", "d", "c", "h" }.OrderBy(w => w, StringComparer.Ordinal)  // c ch d h
$"{1.5:F1}"                                                            // "1,5"     (!)
1.5.ToString(CultureInfo.InvariantCulture)                             // "1.5"
```

| culture-sensitive (careful) | deterministic alternative |
|---|---|
| `OrderBy(s => s)`, `CompareTo`, `string.Compare` | `StringComparer.Ordinal`, `string.CompareOrdinal` |
| `s.StartsWith("x")`, `EndsWith`, `IndexOf(string)` | `s.StartsWith("x", StringComparison.Ordinal)` |
| `ToLower()`, `ToUpper()` | `ToLowerInvariant()` |
| `x.ToString("F1")`, `$"{x:F1}"`, `double.Parse` | `CultureInfo.InvariantCulture` |

| JS | C# |
|---|---|
| `s.split(",")` | `s.Split(',')` |
| `s.split(/\s+/)` | `s.Split((char[]?)null, StringSplitOptions.RemoveEmptyEntries)` |
| `arr.join(",")` | `string.Join(",", arr)` |
| `s.trim()` | `s.Trim()` |
| `s.includes(x)` | `s.Contains(x)` (ordinal) |
| `parseInt(s)` | `int.Parse(s)` (exception) / `int.TryParse(s, out var n)` |
| building up in a loop | `StringBuilder` |

---

## 4. Collections

| JS/TS | C# | note |
|---|---|---|
| `T[]` | `List<T>` | a dynamic array |
| (nothing) | `T[]` | fixed length (like a Go array, but a reference type) |
| `Map<K, V>` | `Dictionary<K, V>` | |
| `Set<T>` | `HashSet<T>` | |
| `readonly T[]` | `IReadOnlyList<T>` | just an interface, can be cast back. Safely: `list.AsReadOnly()` |
| iterable | `IEnumerable<T>` | ≈ anything that goes into `foreach` |

```csharp
List<int> xs = [1, 2, 3];          // collection expression (C# 12) ≈ JS [1, 2, 3]
int[] ys = [..xs, 4];              // spread ≈ [...xs, 4]
List<string> empty = [];
var d = new Dictionary<string, int> { ["a"] = 1 };
```

### Dictionary vs JS Map

| JS `Map` | C# `Dictionary` |
|---|---|
| `m.get(k)` → `undefined` | `d[k]` → **`KeyNotFoundException`** |
| `m.get(k) ?? 0` | `d.GetValueOrDefault(k)` |
| `m.has(k)` + `m.get(k)` | `if (d.TryGetValue(k, out var v)) { … }` |
| `m.set(k, v)` | `d[k] = v` |
| `m.delete(k)` | `d.Remove(k)` |
| `m.size` | `d.Count` |
| insertion order (guaranteed) | **not guaranteed**. In practice it holds until you delete. Don't rely on it |

Idioms:

```csharp
counts[w] = counts.GetValueOrDefault(w) + 1;                  // JS: m.set(w, (m.get(w) ?? 0) + 1)

if (!groups.TryGetValue(k, out var list))                     // JS: (groups[k] ??= []).push(v)
    groups[k] = list = [];
list.Add(v);
```

---

## 5. LINQ ≈ JS Array methods

| JS | LINQ |
|---|---|
| `map` | `Select` |
| `filter` | `Where` |
| `reduce` | `Aggregate` |
| `flatMap` | `SelectMany` |
| `find` | `First(pred)` (exception) / `FirstOrDefault(pred)` |
| `some` / `every` | `Any` / `All` |
| `includes` | `Contains` |
| `length` | `Count()` |
| `slice(a, b)` | `Skip(a).Take(b - a)` |
| `toSorted((a, b) => …)` | `OrderBy(x => x.A).ThenByDescending(x => x.B)` (stable) |
| `[...new Set(arr)]` | `Distinct()` |
| `Object.groupBy` | `GroupBy(x => x.Key)` |
| `Object.fromEntries` | `ToDictionary(x => x.K, x => x.V)` |
| `Math.max(...arr.map(f))` | `Max(f)` |
| the element with the max value | `MaxBy(f)` (returns `null` if empty) |
| sum | `Sum(f)` |
| chunks of n | `Chunk(n)` |
| `Array.from({ length: n }, …)` | `Enumerable.Range(0, n).Select(…)` |

**Difference from JS: LINQ is lazy.** `Where`/`Select` run nothing until you iterate the result. Every further iteration runs the whole chain again. If you want to compute the result once and keep it, call `.ToList()` / `.ToArray()` / `.ToDictionary()`.

---

## 6. Classes, properties, initialization

```csharp
public class Lane
{
    public string Id { get; }                          // read-only property
    public State State { get; private set; }           // read-only from outside
    private readonly List<State> _history = [];

    public Lane(string id) => Id = id;

    public IReadOnlyList<State> History => _history.AsReadOnly();   // computed property ≈ get history()
}

public class Options
{
    public required string BaseUrl { get; init; }      // required, settable only at creation
    public int Retries { get; init; } = 3;
}
var o = new Options { BaseUrl = "https://…" };        // object initializer ≈ a JS literal

class Service(ILogger log) { void Run() => log.Info("…"); }   // primary constructor (C# 12)

public string Name
{
    get;
    set => field = value.Trim();                       // `field` = the hidden backing field (C# 14)
}
```

- Inheritance: `class B : A`, `virtual`/`override`, `abstract`. In practice composition and interfaces are preferred.
- Extension methods: `static class X { public static int Double(this int n) => n * 2; }` and then `5.Double()`.
- C# 14 adds `extension` blocks, which can also declare extension properties: `static class X { extension(int n) { public bool IsEven => n % 2 == 0; } }` and then `4.IsEven`. Classic `this` extension methods keep working and are what you'll see in most code.

---

## 7. Interfaces, pattern matching

```csharp
public interface ISensor { Severity Evaluate(); }
public record CoSensor(double Ppm) : ISensor            // declared explicitly!
{
    public Severity Evaluate() => Ppm switch
    {
        < 50  => Severity.Ok,
        < 150 => Severity.Warn,
        _     => Severity.Alarm,
    };
}

public enum Severity { Ok, Warn, Alarm }                 // an enum is an int under the hood

var text = sensor switch                                 // ≈ TS narrowing via switch
{
    CoSensor { Ppm: > 100 } c => $"high {c.Ppm}",       // type + property pattern
    CoSensor c                => $"co {c.Ppm}",
    null                      => "none",
    _                         => "other",
};

var next = (state, action) switch                        // switch on a tuple
{
    (State.Open, Action.Close) => State.Closed,
    _ => throw new InvalidOperationException(),
};

if (obj is CoSensor { Ppm: var p }) { }                  // is-pattern ≈ instanceof + destructuring
```

---

## 8. Exceptions

They work like in JS, they're just typed and `catch` filters by type:

```csharp
public class ConfigException(string msg, int line, Exception? inner = null)
    : Exception(msg, inner) { public int Line { get; } = line; }

try { … }
catch (ConfigException ex) when (ex.Line > 0) { … }    // filter ≈ if (e instanceof X && …)
catch (IOException ex) { throw new ConfigException("read failed", 0, ex); }   // inner ≈ { cause }
```

- `throw;` rethrows the exception with its stack trace preserved. `throw ex;` wipes it.
- Expected failures use the **Try pattern**: `bool TryParse(string s, out int value)`. It's the equivalent of Go's `(v, ok)`.

---

## 9. Lambdas, delegates, generics

| JS/TS | C# |
|---|---|
| `(x) => x * 2` | `x => x * 2` |
| type `(x: number) => number` | `Func<int, int>` |
| type `(x: number) => void` | `Action<int>` |
| type `(x: T) => boolean` | `Func<T, bool>` / `Predicate<T>` |
| `function f<T extends X>(…)` | `T F<T>(…) where T : X` |
| `(...nums: number[])` | `params int[] nums` |

```csharp
public class Stack<T> { private readonly List<T> _items = []; }
public static T Max<T>(T a, T b) where T : IComparable<T> => a.CompareTo(b) >= 0 ? a : b;
public static T Sum<T>(IEnumerable<T> xs) where T : INumber<T>   // generic math (.NET 7+)
{
    var s = T.Zero; foreach (var x in xs) s += x; return s;
}
public bool TryPop([MaybeNullWhen(false)] out T item) { … }
```

Constraints: `class`, `struct`, `notnull`, `new()`, an interface or a base class.

---

## 10. Iterators ≈ JS generators

| JS | C# |
|---|---|
| `function* f()` | a method returning `IEnumerable<T>` with `yield return` |
| `yield x` | `yield return x;` |
| `return` in a generator | `yield break;` |
| `for (const x of gen)` | `foreach (var x in seq)` |
| `async function*` + `for await` | `IAsyncEnumerable<T>` + `await foreach` |

**Trap:** in a method with `yield`, the whole body, including argument validation, runs only on the first `MoveNext`. If you want an immediate exception, split it into a public method (validation) and a local function with `yield`.

---

## 11. async/await ≈ Promise

| JS | C# |
|---|---|
| `Promise<T>` | `Task<T>` (hot, runs right after creation, like a Promise) |
| `Promise<void>` | `Task` |
| `async function f(): Promise<T>` | `async Task<T> FAsync()` |
| `Promise.all([…])` | `Task.WhenAll(…)`: `await` throws **only the first** exception. All of them are in `task.Exception` |
| `Promise.race([…])` | `Task.WhenAny(…)` (returns the completed **Task**, not the value) |
| `Promise.any` | not built in, write a loop with `WhenAny` |
| `AbortController` | `CancellationTokenSource` |
| `signal` | `CancellationToken` (passed as the last parameter) |
| `setTimeout` | `await Task.Delay(ms, token)` |
| `Date.now()` in a test | `TimeProvider` (can be swapped for a fake) |

Difference from JS: **C# runs on multiple threads.** Shared state must be protected:

```csharp
private readonly Lock _gate = new();         // System.Threading.Lock (.NET 9+), before that `object`
lock (_gate) { … }                           // await inside lock doesn't work
private readonly SemaphoreSlim _sem = new(3);   // async-friendly parallelism limit
await _sem.WaitAsync(ct); try { … } finally { _sem.Release(); }
```

- Never `async void` (except event handlers). Exceptions from it crash the process.
- Don't block with `.Result` / `.Wait()`. It risks a deadlock, use `await`.

---

## 12. Common traps (summary)

- Sorting and comparing strings is culture-sensitive, hence `StringComparer.Ordinal`.
- `$"{x:F1}"` in sk-SK gives a decimal comma.
- `dict[k]` on a missing key throws.
- LINQ is lazy and re-evaluates on every iteration.
- An `IReadOnlyList` returned from a `List` can be cast back, hence `AsReadOnly()`.
- `class ==` compares references, `record ==` compares values.
- Integer division `5/2 == 2`.
- `MaxBy`/`FirstOrDefault` return `null`/`default` on an empty collection.
- `await Task.WhenAll` shows only the first exception.
- `async void`, `.Result`, `await` inside `lock`.
- Order in a `Dictionary` is not guaranteed.

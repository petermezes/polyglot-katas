# Pattern and syntax excercises

10 short exercises. Each one focuses on a single pattern and has the same (or a very similar) task. These are not algorithm puzzles. The point is how a given thing is written idiomatically in each language.

## Structure

```
exercises/       skeletons to solve: go/, csharp/, python/, rust/
tests/           the tests, one copy shared by exercises/ and solutions/
solutions/       reference solutions with comments
TASKS.md         what each exercise practices and what to watch out for
CHEATSHEET_*.md  cheat sheets: Go, C#, Python and Rust for JS/TS developers
```

The skeletons contain a ready-made API (types, signatures, doc comments with tips) and the bodies are `panic("TODO")` / `throw new NotImplementedException()` / `raise NotImplementedError("TODO")` / `todo!()`. Your job is to get the tests green.

You edit code in `exercises/<language>/` and run the tests from `tests/<language>/`. By default the tests run against `exercises/`. Each language has a switch that runs the same tests against `solutions/` instead (see [Running](#running)).

## Exercises

| # | Topic | Go `exercises/go/` | C# `exercises/csharp/Exercises/` | Python `exercises/python/` | Rust `exercises/rust/src/` |
|---|---|---|---|---|---|
| 01 | Collections | `01-collections/collections.go` | `Ex01Collections/WordStats.cs` | `ex01_collections/word_stats.py` | `ex01_collections.rs` |
| 02 | Errors | `02-errors/errors.go` | `Ex02Errors/ConfigParser.cs` | `ex02_errors/config_parser.py` | `ex02_errors.rs` |
| 03 | Interfaces | `03-interfaces/interfaces.go` | `Ex03Interfaces/Sensors.cs` | `ex03_interfaces/sensors.py` | `ex03_interfaces.rs` |
| 04 | Generics | `04-generics/generics.go` | `Ex04Generics/Generics.cs` | `ex04_generics/generics.py` | `ex04_generics.rs` |
| 05 | Queries | `05-queries/queries.go` | `Ex05Queries/EventQueries.cs` | `ex05_queries/event_queries.py` | `ex05_queries.rs` |
| 06 | State machine | `06-states/states.go` | `Ex06States/Lane.cs` | `ex06_states/lane.py` | `ex06_states.rs` |
| 07 | Options / builder | `07-options/options.go` | `Ex07Options/ClientBuilder.cs` | `ex07_options/client_config.py` | `ex07_options.rs` |
| 08 | TTL cache | `08-cache/cache.go` | `Ex08Cache/TtlCache.cs` | `ex08_cache/ttl_cache.py` | `ex08_cache.rs` |
| 09 | Iterators | `09-iterators/iterators.go` | `Ex09Iterators/Sequences.cs` | `ex09_iterators/sequences.py` | `ex09_iterators.rs` |
| 10 | Concurrency | `10-concurrency/concurrency.go` | `Ex10Concurrency/Concurrency.cs` | `ex10_concurrency/concurrency.py` | `ex10_concurrency.rs` |

The solutions have the same paths under `solutions/`. The tests for exercise 01 are `tests/go/01-collections/`, `tests/csharp/Exercises.Tests/Ex01CollectionsTests.cs`, `tests/python/test_ex01_collections.py` and `tests/rust/tests/ex01_collections.rs`, and likewise for the others.

The order 01 → 10 goes from simpler to harder. 08 and 10 are the most demanding.

## Requirements

- **Go 1.23+** (exercise 09 uses range-over-func iterators)
- **.NET 10 SDK or newer** (the projects target `net10.0`, the tests also run on a newer runtime)
- **Python 3.12+** (generics via `class Stack[T]`) and **pytest**
- **Rust 1.85+** (edition 2024). Install it with [rustup](https://rustup.rs) (recommended, manages toolchains and updates) or `brew install rust`. No crates are needed, only the standard library.

## Running

**Go**

```bash
cd tests/go
go test ./01-collections/                # one exercise
go test -run TestTopN ./01-collections/  # one test
go test ./...                            # everything
go test -race ./10-concurrency/          # concurrency with the race detector
go test -modfile=solutions.mod ./...     # the same tests against solutions/go
```

`panic("TODO")` brings down the whole test package, so you always see only the first unfinished test. That's fine, you go step by step.

**C#**

```bash
cd tests/csharp
dotnet test --filter "FullyQualifiedName~Ex01"                          # one exercise
dotnet test --filter "FullyQualifiedName~Ex01&FullyQualifiedName~TopN"  # one test
dotnet test                                                             # everything
dotnet test -p:Solutions=true                                           # the same tests against solutions/csharp
```

**Python**

Create a virtual environment in `tests/python` and install pytest once:

```bash
cd tests/python
python -m venv .venv
.venv\Scripts\activate          # Windows (PowerShell)
source .venv/bin/activate       # macOS / Linux
pip install pytest
```

Then, with `.venv` activated, still in `tests/python`:

```bash
python -m pytest test_ex01_collections.py           # one exercise
python -m pytest test_ex01_collections.py -k top_n  # tests whose name contains "top_n"
python -m pytest                                    # everything
python -m pytest -x                                 # stop at the first failing test
python -m pytest --solutions                        # the same tests against solutions/python
```

If you use [uv](https://docs.astral.sh/uv/), just `cd tests/python` and `uv run pytest test_ex01_collections.py` (it installs pytest by itself).

Each test fails on its own here, so you see right away how much is left. pytest hides `print()` output. Add `-s` if you want to see it.

**Rust**

```bash
cd tests/rust
cargo test --test ex01_collections          # one exercise
cargo test --test ex01_collections top_n    # tests whose name contains "top_n"
cargo test --no-fail-fast                   # everything
cargo test --features solutions             # the same tests against solutions/rust
```

Each `todo!()` fails only its own test, like in Python. Without `--no-fail-fast`, `cargo test` stops after the first exercise with a failing test. `println!` output is hidden, add `-- --nocapture` to see it.

## Recommended workflow

1. Read the section in `TASKS.md`.
2. Solve the exercise in one language first, then right away in the next one. That's how the differences stick best.
3. Only once the tests pass, compare with `solutions/`. The solutions are not the only right way, but they show the idiomatic style and contain comments about the traps.
4. Bonus: try writing one or two tests for cases that aren't covered (in `tests/`, check them with the solutions switch).

## Editor setup

- **C#:** open `tests/csharp/Exercises.slnx`. It contains the test project and `exercises/csharp/Exercises`.
- **Python:** `tests/python/pyproject.toml` tells Pylance / Pyright to resolve the test imports from `exercises/python`.
- **Rust:** rust-analyzer finds `tests/rust` when you open that folder. It indexes `exercises/rust` and `solutions/rust` too, because they are its dependencies. If you open the repository root and rust-analyzer doesn't pick it up, add `"rust-analyzer.linkedProjects": ["tests/rust/Cargo.toml"]` to your editor settings (in VS Code: `.vscode/settings.json`).
- **Go:** gopls handles the separate modules (`exercises/go`, `solutions/go`, `tests/go`) on its own.

## How the solutions switch works

The skeletons and the solutions have the same API, so one set of tests serves both. Each language picks the code under test differently:

- **Go:** `tests/go` is a separate module. Its `go.mod` points the `exercises` module to `exercises/go` with a `replace`, `solutions.mod` points it to `solutions/go`, and `-modfile` chooses between them. The tests are an external test package (`package collections_test`) with a dot import, so they only see the exported API, like any other caller.
- **C#:** the test project's `ProjectReference` depends on the `Solutions` property: `exercises/csharp/Exercises` by default, `solutions/csharp/Exercises` with `-p:Solutions=true`.
- **Python:** `tests/python/conftest.py` puts `exercises/python` (or `solutions/python` with `--solutions`) on `sys.path`. The header of the pytest output shows which one is being tested.
- **Rust:** Cargo can't switch a path dependency with a feature, so `tests/rust` depends on both crates. Its small library `under_test` re-exports `exercises` by default and `solutions` with `--features solutions`, and the tests `use under_test::...`.

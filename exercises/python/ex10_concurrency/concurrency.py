"""Exercise 10: concurrency — asyncio, Semaphore, TaskGroup, cancellation via cancel(), ExceptionGroup.

The difference from Go/C#: asyncio passes no context or CancellationToken around. Cancellation
is task.cancel(), which raises CancelledError in the coroutine at the next `await`.
"""

import asyncio
from collections.abc import Awaitable, Callable, Sequence


async def process_all[T, R](
    inputs: Sequence[T],
    workers: int,
    f: Callable[[T], Awaitable[R]],
) -> list[R]:
    """Processes all inputs, at most `workers` at a time (treat workers < 1 as 1).
    Results are in input order. On the first error it cancels the other tasks, starts no new ones
    and raises that original exception (not ExceptionGroup). Cancellation from outside must propagate.

    Tip:
      - asyncio.Semaphore(workers) and inside the task `async with semaphore: ...`
      - results = [None] * len(inputs); each task writes its own index
      - async with asyncio.TaskGroup() as tg: tg.create_task(...) — when one task fails,
        TaskGroup cancels the others and waits for them
      - TaskGroup always wraps errors in an ExceptionGroup: except ExceptionGroup as g: raise g.exceptions[0]
      - never catch and swallow CancelledError, or cancellation from outside stops working
    """
    raise NotImplementedError("TODO")


async def first_success[R](*fns: Callable[[], Awaitable[R]]) -> R:
    """Starts all functions at once and returns the first successful result; cancels the rest.
    If all fail, raises an ExceptionGroup with all the errors. ValueError with no functions.

    Tip:
      - tasks = [asyncio.create_task(fn()) for fn in fns]
      - for next_done in asyncio.as_completed(tasks): — you get them in the order they finish
      - try/finally: in finally, task.cancel() on all and await asyncio.gather(*tasks, return_exceptions=True),
        so you don't leave dangling tasks behind (like a goroutine leak)
      - ExceptionGroup("message", list_of_errors)
    """
    raise NotImplementedError("TODO")

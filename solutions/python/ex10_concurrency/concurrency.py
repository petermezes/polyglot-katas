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
    """Processes all inputs, at most `workers` at a time. Results in input order.
    On the first error it cancels the other tasks and raises that original exception (not ExceptionGroup)."""
    semaphore = asyncio.Semaphore(max(workers, 1))
    results: list[R] = [None] * len(inputs)  # type: ignore[list-item]  # each index is written by exactly one task

    async def run(i: int, item: T) -> None:
        async with semaphore:  # at most `workers` tasks are inside at once
            results[i] = await f(item)

    try:
        # TaskGroup (3.11+): if one task fails, it cancels the others and waits for them.
        # Tasks still waiting on the semaphore get cancelled before they call f.
        async with asyncio.TaskGroup() as tg:
            for i, item in enumerate(inputs):
                tg.create_task(run(i, item))
    except ExceptionGroup as group:
        # TaskGroup always wraps errors in an ExceptionGroup. The spec wants the original exception.
        raise group.exceptions[0] from None
    # Cancellation from outside (e.g. asyncio.timeout) arrives here as CancelledError, not ExceptionGroup.
    # Never catch and swallow it, or cancellation stops working.
    return results


async def first_success[R](*fns: Callable[[], Awaitable[R]]) -> R:
    """Starts them all at once and returns the first successful result; cancels the rest.
    If all fail, raises an ExceptionGroup with all the errors. ValueError with no functions."""
    if not fns:
        raise ValueError("no functions to run")

    tasks = [asyncio.create_task(fn()) for fn in fns]
    errors: list[Exception] = []
    try:
        for next_done in asyncio.as_completed(tasks):  # in the order they finish
            try:
                return await next_done
            except Exception as err:  # CancelledError is a BaseException, it won't land here
                errors.append(err)
        raise ExceptionGroup("all attempts failed", errors)
    finally:
        # Runs on return and on exception: cancel the rest and wait until they really finish,
        # so we don't leave dangling tasks behind (like a goroutine leak).
        for task in tasks:
            task.cancel()
        await asyncio.gather(*tasks, return_exceptions=True)

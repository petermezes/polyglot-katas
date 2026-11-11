"""Exercise 07: configuration — keyword arguments, frozen dataclass, __post_init__, dataclasses.replace.

Python needs neither a builder nor functional options: named arguments with default
values do the same thing right in the language. `replace()` is the equivalent of the C# `with` expression.
"""

from collections.abc import Mapping
from dataclasses import KW_ONLY, dataclass, field, replace
from datetime import timedelta
from types import MappingProxyType


class InsecureURLError(ValueError):
    pass


class InvalidOptionError(ValueError):
    pass


@dataclass(frozen=True)
class ClientConfig:
    """HTTP client configuration. Usage: ClientConfig("https://api", retries=0, timeout=...)."""

    base_url: str
    _: KW_ONLY  # everything after this line can be passed by name only (like * in a function signature)

    # TODO: add fields with default values:
    #   timeout: timedelta    -> 30 seconds
    #   retries: int          -> 3
    #   user_agent: str       -> "exercises/1.0"
    #   headers: Mapping[str, str] -> empty dict
    # Careful: dataclass rejects `headers: dict = {}` (shared mutable default).
    # Tip: field(default_factory=dict).

    def __post_init__(self) -> None:
        """Validation after creation (also runs on replace()):

          - base_url without "https://"  -> InsecureURLError
          - timeout <= 0                 -> InvalidOptionError
          - retries outside 0..10        -> InvalidOptionError

        Copy the headers (the caller may keep changing their dict) and wrap them in a read-only
        MappingProxyType view. frozen=True only protects attributes, not the dict's contents.
        Tip: in __post_init__ you can write to a frozen dataclass only via
        object.__setattr__(self, "headers", ...).
        """
        raise NotImplementedError("TODO")

    def with_header(self, key: str, value: str) -> "ClientConfig":
        """A new configuration with the header added. The original is unchanged.

        Tip: replace(self, headers={**self.headers, key: value}).
        """
        raise NotImplementedError("TODO")

    def slower(self, factor: float) -> "ClientConfig":
        """A copy with the timeout multiplied by factor (a timedelta can be multiplied by a number)."""
        raise NotImplementedError("TODO")

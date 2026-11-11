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
    base_url: str
    _: KW_ONLY  # everything after this line can be passed by name only: ClientConfig(url, retries=0)
    timeout: timedelta = timedelta(seconds=30)
    retries: int = 3
    user_agent: str = "exercises/1.0"
    # Trap: dataclass rejects `headers: dict = {}` (shared mutable default). Use default_factory.
    headers: Mapping[str, str] = field(default_factory=dict)

    def __post_init__(self) -> None:
        """Validation after creation. Also runs on replace(), so an invalid copy can't be created."""
        if not self.base_url.startswith("https://"):
            raise InsecureURLError(f"URL must start with https://: {self.base_url}")
        if self.timeout <= timedelta(0):
            raise InvalidOptionError(f"timeout {self.timeout}: must be positive")
        if not 0 <= self.retries <= 10:
            raise InvalidOptionError(f"retries {self.retries}: allowed 0 to 10")
        # frozen=True only protects attribute assignment, not the dict's contents. So we copy it
        # (the caller may keep changing their dict) and wrap it in a read-only view.
        # object.__setattr__ is the only way to write to a frozen dataclass in __post_init__.
        object.__setattr__(self, "headers", MappingProxyType(dict(self.headers)))

    def with_header(self, key: str, value: str) -> "ClientConfig":
        """A new configuration with the header added. The original is unchanged."""
        return replace(self, headers={**self.headers, key: value})

    def slower(self, factor: float) -> "ClientConfig":
        """A copy with the timeout multiplied by factor."""
        return replace(self, timeout=self.timeout * factor)

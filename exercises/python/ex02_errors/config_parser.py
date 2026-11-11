"""Exercise 02: errors — exception hierarchy, attributes on exceptions, chaining with `raise ... from`."""

from collections.abc import Mapping


class ConfigError(Exception):
    """Base of all configuration errors. The caller can catch the whole category at once
    (`except ConfigError`), similar to errors.Is on a common ancestor."""


class ConfigSyntaxError(ConfigError):
    """A line without '='. Attributes line, text. Message: line 3: missing '=' in "no equals sign"

    (Not SyntaxError: that's the built-in exception for errors in Python code.)
    Tip: super().__init__(message), then self.line = line, ...
    """

    def __init__(self, line: int, text: str) -> None:
        raise NotImplementedError("TODO")


class EmptyKeyError(ConfigError):
    """Attribute line. Message exactly: line 2: empty key"""

    def __init__(self, line: int) -> None:
        raise NotImplementedError("TODO")


class DuplicateKeyError(ConfigError):
    """Attributes line, key. Message e.g.: line 2: duplicate key 'a'"""

    def __init__(self, line: int, key: str) -> None:
        raise NotImplementedError("TODO")


class MissingKeyError(ConfigError):
    """Attribute key. Message e.g.: missing key 'port'"""

    def __init__(self, key: str) -> None:
        raise NotImplementedError("TODO")


class ConfigValueError(ConfigError):
    """The value cannot be converted. Attributes key, value. The original ValueError must be in __cause__."""

    def __init__(self, key: str, value: str) -> None:
        raise NotImplementedError("TODO")


def parse_config(text: str) -> dict[str, str]:
    """Parses "key = value" line by line (numbered from 1).
    Empty lines and lines starting with '#' are ignored. Both key and value are trimmed of whitespace.

      - line without '='   -> ConfigSyntaxError(line, trimmed line)
      - empty key          -> EmptyKeyError
      - key already exists -> DuplicateKeyError

    Tip: enumerate(text.splitlines(), start=1), str.strip(), str.partition("=")
    (like strings.Cut: returns (before, separator, after), separator is "" if missing).
    """
    raise NotImplementedError("TODO")


def get_int(cfg: Mapping[str, str], key: str) -> int:
    """The key's value as an int.

      - missing key    -> MissingKeyError
      - invalid number -> ConfigValueError, the original ValueError from int() in __cause__

    Tip: try/except KeyError and try/except ValueError as err: raise ConfigValueError(...) from err.
    `from None`, on the other hand, hides the original exception.
    """
    raise NotImplementedError("TODO")


def try_get_int(cfg: Mapping[str, str], key: str) -> int | None:
    """Python's take on the Try pattern: no exception, None on failure.

    Tip: use get_int and catch only ConfigError (not a bare `except:`).
    """
    raise NotImplementedError("TODO")

"""Exercise 02: errors — exception hierarchy, attributes on exceptions, chaining with `raise ... from`."""

from collections.abc import Mapping


class ConfigError(Exception):
    """Base of all configuration errors. The caller can catch the whole category at once
    (`except ConfigError`), similar to errors.Is on a common ancestor."""


class ConfigSyntaxError(ConfigError):
    """A line without '='. Not SyntaxError: that's the built-in exception for errors in Python code."""

    def __init__(self, line: int, text: str) -> None:
        super().__init__(f"line {line}: missing '=' in \"{text}\"")
        self.line = line
        self.text = text


class EmptyKeyError(ConfigError):
    def __init__(self, line: int) -> None:
        super().__init__(f"line {line}: empty key")
        self.line = line


class DuplicateKeyError(ConfigError):
    def __init__(self, line: int, key: str) -> None:
        super().__init__(f"line {line}: duplicate key '{key}'")
        self.line = line
        self.key = key


class MissingKeyError(ConfigError):
    def __init__(self, key: str) -> None:
        super().__init__(f"missing key '{key}'")
        self.key = key


class ConfigValueError(ConfigError):
    """The value cannot be converted. The original exception is in __cause__ (like %w in Go, InnerException in C#)."""

    def __init__(self, key: str, value: str) -> None:
        super().__init__(f"key '{key}': invalid value '{value}'")
        self.key = key
        self.value = value


def parse_config(text: str) -> dict[str, str]:
    """Parses "key = value" line by line. Empty lines and lines starting with '#' are ignored."""
    cfg: dict[str, str] = {}
    for line_no, raw in enumerate(text.splitlines(), start=1):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        key, sep, value = line.partition("=")  # like strings.Cut: sep is "" if '=' is missing
        if not sep:
            raise ConfigSyntaxError(line_no, line)
        key, value = key.strip(), value.strip()
        if not key:
            raise EmptyKeyError(line_no)
        if key in cfg:
            raise DuplicateKeyError(line_no, key)
        cfg[key] = value
    return cfg


def get_int(cfg: Mapping[str, str], key: str) -> int:
    """The key's value as an int. MissingKeyError / ConfigValueError with the original ValueError in __cause__."""
    try:
        raw = cfg[key]
    except KeyError:
        # `from None` hides the original KeyError from the traceback: for the caller it's an implementation detail.
        raise MissingKeyError(key) from None
    try:
        return int(raw)
    except ValueError as err:
        # `from err`, on the other hand, keeps it in __cause__ — the caller can get to it.
        raise ConfigValueError(key, raw) from err


def try_get_int(cfg: Mapping[str, str], key: str) -> int | None:
    """Python's take on the Try pattern: no exception, None on failure."""
    try:
        return get_int(cfg, key)
    except ConfigError:  # catch only what we can handle, not a bare `except:`
        return None

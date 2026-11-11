import pytest

from ex02_errors.config_parser import (
    ConfigError,
    ConfigSyntaxError,
    ConfigValueError,
    DuplicateKeyError,
    EmptyKeyError,
    MissingKeyError,
    get_int,
    parse_config,
    try_get_int,
)

VALID_CONFIG = """
# server settings
host = tunel.example.no
port = 8080

timeout=30
"""


def test_parse_config_valid():
    assert parse_config(VALID_CONFIG) == {"host": "tunel.example.no", "port": "8080", "timeout": "30"}


def test_parse_config_syntax_error():
    with pytest.raises(ConfigSyntaxError) as exc_info:
        parse_config("a = 1\n\nno equals sign")
    err = exc_info.value
    assert (err.line, err.text) == (3, "no equals sign")
    assert str(err) == "line 3: missing '=' in \"no equals sign\""


def test_parse_config_empty_key():
    with pytest.raises(EmptyKeyError, match=r"^line 2: empty key$"):
        parse_config("a = 1\n = 2")


def test_parse_config_duplicate_key():
    with pytest.raises(DuplicateKeyError) as exc_info:
        parse_config("a = 1\na = 2")
    assert (exc_info.value.line, exc_info.value.key) == (2, "a")


@pytest.mark.parametrize("text", ["no equals sign", "= 1", "a = 1\na = 2"])
def test_all_parse_errors_are_config_errors(text):
    with pytest.raises(ConfigError):
        parse_config(text)


def test_get_int():
    cfg = {"port": "8080", "host": "x"}

    assert get_int(cfg, "port") == 8080

    with pytest.raises(MissingKeyError) as missing:
        get_int(cfg, "missing")
    assert missing.value.key == "missing"

    with pytest.raises(ConfigValueError) as invalid:
        get_int(cfg, "host")
    assert invalid.value.key == "host"
    assert isinstance(invalid.value.__cause__, ValueError), "the original ValueError should be in __cause__ (raise ... from err)"


def test_try_get_int():
    cfg = {"port": "8080", "host": "x"}
    assert try_get_int(cfg, "port") == 8080
    assert try_get_int(cfg, "host") is None
    assert try_get_int(cfg, "missing") is None

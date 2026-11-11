from dataclasses import FrozenInstanceError, replace
from datetime import timedelta

import pytest

from ex07_options.client_config import ClientConfig, InsecureURLError, InvalidOptionError


def test_defaults():
    c = ClientConfig("https://api.example.no")
    assert c.timeout == timedelta(seconds=30)
    assert c.retries == 3
    assert c.user_agent == "exercises/1.0"
    assert dict(c.headers) == {}


def test_options():
    c = ClientConfig(
        "https://api.example.no",
        timeout=timedelta(seconds=5),
        retries=0,
        headers={"Accept": "application/json", "X-Tunnel": "Løvstakk"},
        user_agent="vegvokter-test",
    )
    assert (c.timeout, c.retries, c.user_agent) == (timedelta(seconds=5), 0, "vegvokter-test")
    assert c.headers["X-Tunnel"] == "Løvstakk"
    assert len(c.headers) == 2


def test_options_are_keyword_only():
    with pytest.raises(TypeError):
        ClientConfig("https://x", timedelta(seconds=5))  # type: ignore[misc]


@pytest.mark.parametrize(
    ("kwargs", "error"),
    [
        ({"base_url": "http://api.example.no"}, InsecureURLError),
        ({"base_url": "https://x", "timeout": timedelta(0)}, InvalidOptionError),
        ({"base_url": "https://x", "retries": 11}, InvalidOptionError),
        ({"base_url": "https://x", "retries": -1}, InvalidOptionError),
    ],
)
def test_invalid(kwargs, error):
    with pytest.raises(error):
        ClientConfig(**kwargs)


def test_errors_are_value_errors():
    with pytest.raises(ValueError):
        ClientConfig("http://x")


def test_frozen():
    c = ClientConfig("https://x")
    with pytest.raises(FrozenInstanceError):
        c.retries = 9  # type: ignore[misc]


def test_headers_are_copied_and_read_only():
    mine = {"A": "1"}
    c = ClientConfig("https://x", headers=mine)
    mine["B"] = "2"
    assert "B" not in c.headers, "the config should copy the headers"
    with pytest.raises(TypeError):
        c.headers["C"] = "3"  # type: ignore[index]


def test_with_header_returns_new_config():
    orig = ClientConfig("https://x", headers={"A": "1"})
    changed = orig.with_header("B", "2")
    assert dict(changed.headers) == {"A": "1", "B": "2"}
    assert dict(orig.headers) == {"A": "1"}


def test_replace_and_slower():
    orig = ClientConfig("https://x")
    assert orig.slower(2).timeout == timedelta(seconds=60)
    assert orig.timeout == timedelta(seconds=30)
    assert replace(orig, retries=5).retries == 5
    with pytest.raises(InvalidOptionError):
        replace(orig, retries=11)  # validation also runs on replace


def test_equality():
    assert ClientConfig("https://x", retries=1) == ClientConfig("https://x", retries=1)
    assert ClientConfig("https://x") != ClientConfig("https://x", retries=1)

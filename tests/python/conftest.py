"""Chooses which code the tests import.

Default: exercises/python. With `--solutions`: solutions/python.
Both have the same packages (ex01_collections, ...), so the tests don't change.
"""

import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]


def pytest_addoption(parser):
    parser.addoption(
        "--solutions",
        action="store_true",
        help="run the tests against solutions/python instead of exercises/python",
    )


def _code_root(config) -> Path:
    return REPO_ROOT / ("solutions" if config.getoption("--solutions") else "exercises") / "python"


def pytest_configure(config):
    sys.path.insert(0, str(_code_root(config)))


def pytest_report_header(config):
    return f"code under test: {_code_root(config).relative_to(REPO_ROOT)}"

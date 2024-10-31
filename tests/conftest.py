from pathlib import Path

import pytest


@pytest.fixture(scope="session")
def fixture_dir() -> Path:
    return Path(__file__).parents[0].resolve() / "fixtures"

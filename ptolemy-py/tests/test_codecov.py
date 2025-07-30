"""
Test codecov.

NOTE: This will be deleted once we have actual unit tests (PR waiting for review)
"""

import pytest

def test_codecov():
    assert True

@pytest.mark.benchmark
def test_codspeed():
    assert 2 ** 2 == 4

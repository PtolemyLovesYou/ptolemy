"""Test IO object."""

import uuid
import pytest
from pydantic import ValidationError
from ptolemy_client.io import IO

def test_validate_io_success():
    """Validate IO."""

    for field_value in ["field_value", [1, 2, 3, 4, 5], [1.2, 1, 3.4, None]]:
        IO(parent_id=uuid.uuid4(), field_name="field_name", field_value=field_value)

def test_validate_invalid_type():
    """Invalid type should throw error."""

    with pytest.raises(ValidationError, match=r"Invalid type"):
        IO(parent_id=uuid.uuid4(), field_name="field_name", field_value=b"asdf")

@pytest.mark.benchmark
def test_benchmark_io_instantiation():
    IO(parent_id=uuid.uuid4(), field_name="field_name", field_value="foo")

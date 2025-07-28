"""Unit tests for validation logic in `validate_field_value`."""

import pytest
from ptolemy_client._core import validate_field_value

def test_successful_flat_list():
    """Should pass: flat list of length equal to max_size."""
    validate_field_value([1] * 1024, max_size=1024)

def test_successful_flat_dict():
    """Should pass: flat dict with <= max_size keys."""
    validate_field_value({str(i): i for i in range(1024)}, max_size=1024)

def test_unsuccessful_flat_list():
    """Should fail: flat list exceeding max_size."""
    with pytest.raises(ValueError):
        validate_field_value([1] * 1026, max_size=1024)

def test_unsuccessful_nested_list():
    """Should fail: nested list expanded beyond max_size."""
    nested = [[1, 2, [1, 2]], [5, 6, 7], [8, 9, 10]]
    with pytest.raises(ValueError):
        validate_field_value(nested * 1024, max_size=1024)

def test_unsuccessful_empty_path():
    """Should fail: values with empty lists should still count as leaf nodes."""
    data = {str(i): [] for i in range(1026)}
    with pytest.raises(ValueError):
        validate_field_value(data, max_size=1024)

def test_unsuccessful_null():
    """Should fail: None values should count as leaves."""
    with pytest.raises(ValueError):
        validate_field_value([None] * 1026, max_size=1024)

def test_successful_nested_list():
    """Should pass: nested list within max_size limit."""
    nested = [[1, 2, [1, 2]], [5, 6, 7], [8, 9, 10]]
    validate_field_value(nested * 3, max_size=1024)

def test_successful_nested_dict():
    """Should pass: nested dict where total leaves are within max_size."""
    nested = [[1, 2, [1, 2]], [5, 6, 7], [8, 9, 10]]
    validate_field_value({i: nested for i in range(10)}, max_size=1024)

def test_max_size():
    """Should fail: max_size must be 2 bit"""
    
    with pytest.raises(OverflowError):
        validate_field_value([1, 2, [1] * 2**16], max_size=2**16)

def test_successful_primitive():
    """Should pass: primitives"""
    for i in [1, 1.2, None, "str", True]:
        validate_field_value(i)

def test_unsuccessful_custom_class():
    """Should fail: unserializable class"""
    my_obj = type("my_type", (), {})()

    with pytest.raises(ValueError):
        validate_field_value(my_obj)

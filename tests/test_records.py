"""Test Records classes."""
import pytest
from uuid import uuid4
from pydantic import ValidationInfo
from tvali.records.types import Record, validate_id
from tvali.records.records import SystemRecord, SubsystemRecord, SubcomponentRecord, ComponentRecord

def test_record_creation():
    """
    Test the creation of records.

    This test creates a new system record, a new subsystem record, a new component
    record, and a new subcomponent record. It then checks that each record is an
    instance of the appropriate class.
    """
    system_record = SystemRecord.new('system_name')
    subsystem_record = system_record.subsystem_record('subsystem_name')
    component_record = subsystem_record.component_record('component_name')
    subcomponent_record = component_record.subcomponent_record('subcomponent_name')

    assert isinstance(system_record, SystemRecord)
    assert isinstance(subsystem_record, SubsystemRecord)
    assert isinstance(component_record, ComponentRecord)
    assert isinstance(subcomponent_record, SubcomponentRecord)

def test_record_validation():
    """
    Test the validation of a Record object.

    This test creates a new record with some parameters and logs some inputs, outputs,
    metadata, and feedback. It then checks that the start and end times are floats,
    that the inputs, outputs, metadata, and feedback are as expected, and that the
    record has the correct start and end times.
    """
    test_record = Record.new('test_record', parameters={'foo': 'bar'})

    with test_record:
        test_record.log_io(
            inputs={'foo': 'bar'},
            outputs={'baz': None},
            metadata={'metadata': 'test'},
            feedback={'feedback': [1, 2, [1, 2, 3]]}
        )

    assert isinstance(test_record.start_time, float)
    assert isinstance(test_record.end_time, float)
    assert test_record.inputs == {'foo': 'bar'}
    assert test_record.outputs == {'baz': None}
    assert test_record.metadata == {'metadata': 'test'}
    assert test_record.feedback == {'feedback': [1, 2, [1, 2, 3]]}

def test_bad_record_validation_metadata():
    """
    Test that a ValueError is raised if metadata is not a dictionary of strings.

    This test creates a new record with a non-serializable class as an input.
    It then attempts to log the inputs to the record. The log_io method should
    raise a ValueError since the inputs are not JSON serializable.

    This test confirms that the log_io method correctly validates metadata.
    """
    test_record = Record.new('test_record', parameters={'foo': 'bar'})
    
    with pytest.raises(ValueError):
        test_record.log_io(metadata={'metadata': 1})

def test_bad_record_validation_io():
    """
    Test that a ValueError is raised if inputs are not JSON serializable.

    This test creates a new record with a non-serializable class as an input.
    It then attempts to log the inputs to the record. The log_io method should
    raise a ValueError since the inputs are not JSON serializable.

    This test confirms that the log_io method correctly validates inputs.
    """
    class BadClass:
        """Non-serializable class."""
        pass

    test_record = Record.new('test_record', parameters={'foo': 'bar'})

    with pytest.raises(ValueError):
        test_record.log_io(inputs={'foo': BadClass()})

def test_record_id_validation():
    """
    Test the validation of a Record ID.

    This test validates a UUID, None, and an invalid ID.
    """
    assert validate_id(uuid4(), None)
    assert validate_id(None, None) is None

    with pytest.raises(ValueError):
        validate_id('asdf', None)

def test_immutability():
    """
    Test that Record objects are immutable after they are started.

    This test creates a new record and logs some inputs, outputs, metadata, and
    feedback. It then checks that attempting to log more inputs, outputs, metadata,
    and feedback raises an AttributeError.
    """
    test_record = Record.new('test_record', parameters={'foo': 'bar'})

    with test_record:
        test_record.log_io(
            inputs={'foo': 'bar'},
            outputs={'baz': None},
            metadata={'metadata': 'test'},
            feedback={'feedback': [1, 2, [1, 2, 3]]}
        )

    with pytest.raises(AttributeError):
        test_record.log_io(
            inputs={'foo': 'bar'},
            outputs={'baz': None},
            metadata={'metadata': 'test'},
            feedback={'feedback': [1, 2, [1, 2, 3]]}
        )

    with pytest.raises(ValueError):
        with test_record:
            pass

def test_error_handling():
    """
    Test that error information is logged correctly.

    This test creates a new record and then raises a ValueError inside the record.
    It then checks that the error_type and error_value fields are set correctly.
    """
    test_record = Record.new('test_record', parameters={'foo': 'bar'})

    try:
        with test_record:
            raise ValueError("test error")
    except ValueError:
        assert test_record.error_type == "ValueError"
        assert isinstance(test_record.error_value, str)

"""Test Log objects."""

from datetime import datetime
import uuid
import pytest
from pydantic import ValidationError
import tvali
from tvali.log.types import validate_datetime_field, validate_uuid
from .runner import TvaliTestRunner

VALID_IO = {"foo": "bar", "baz": 1, "qux": [{"foo": "bar"}], "null_value": None}


class TestLogObjects(TvaliTestRunner):
    """Test Log objects."""
    @pytest.mark.asyncio
    async def test_instantiation(self):
        """Test instantiation of Log objects.

        This test ensures that each log object can be instantiated correctly, and that
        the correct types are returned.
        """
        system_log = await tvali.SystemLog.new(name="MySystemName", parameters=VALID_IO)

        assert isinstance(system_log, tvali.SystemLog)

        subsystem_log = await system_log.subsystem("MySubsystemName", parameters=VALID_IO)

        assert isinstance(subsystem_log, tvali.SubsystemLog)

        component_log = await subsystem_log.component(name="MyComponentName2")

        assert isinstance(component_log, tvali.ComponentLog)

        subcomponent_log = await component_log.subcomponent(name="MySubcomponentName")

        assert isinstance(subcomponent_log, tvali.SubcomponentLog)

    @pytest.mark.asyncio
    async def test_version_enviornment(self):
        """Test that SystemLog objects have correct version and environment.

        This test ensures that `tvali.SystemLog` objects have the correct version and
        environment attributes.
        """
        system_log = await tvali.SystemLog.new(name="MySystemName", parameters=VALID_IO)

        assert system_log.version == self.VERSION
        assert system_log.environment == self.ENV

    @pytest.mark.asyncio
    async def test_event_execution(self):
        """Test event execution.

        This test ensures that the `execute` method can be used as a context manager,
        and that the start and end times are correctly set.
        """
        system_log = await tvali.SystemLog.new("test")

        with system_log.execute():
            pass

        assert isinstance(system_log.start_time, datetime)
        assert isinstance(system_log.end_time, datetime)
        assert system_log.end_time > system_log.start_time
        assert system_log.error_type is None
        assert system_log.error_content is None

    @pytest.mark.asyncio
    async def test_event_error(self):
        """Test that errors are correctly captured and stored.

        This test ensures that any exceptions raised during event execution are
        correctly captured and stored in the log object.
        """
        system_log = await tvali.SystemLog.new("test")
        with pytest.raises(ValueError):
            with system_log.execute():
                raise ValueError("test")

        assert system_log.error_type == "ValueError"
        assert isinstance(system_log.error_content, str)

        assert isinstance(system_log.start_time, datetime)
        assert isinstance(system_log.end_time, datetime)
        assert system_log.end_time > system_log.start_time

    @pytest.mark.asyncio
    async def test_event(self):
        """Test that the event() method returns a dictionary.

        This test ensures that the `event` method returns a dictionary, which is
        used to store the event data in the database.
        """
        system_log = await tvali.SystemLog.new("test")
        assert isinstance(system_log.event(), dict)

    @pytest.mark.asyncio
    async def test_timestamp_constraints(self):
        """Test timestamp constraints.

        This test ensures that the start and stop times cannot be set after they
        have already been set.
        """
        system_log = await tvali.SystemLog.new("test")
        system_log.start()
        system_log.stop()

        with pytest.raises(ValueError):
            system_log.start()

        with pytest.raises(ValueError):
            system_log.stop()

    @pytest.mark.asyncio
    async def test_duplicate_logs(self):
        """Test that duplicate logs are not allowed.

        This test ensures that the same log cannot be written twice. This is
        important for preventing duplicate data from being written to the
        database.
        """
        system_log = await tvali.SystemLog.new("test")

        for i in ["inputs", "outputs", "feedback", "metadata"]:
            system_log.log(**{i: VALID_IO})
            assert system_log.__dict__[i] == VALID_IO
            with pytest.raises(ValueError):
                system_log.log(**{i: VALID_IO})

    @pytest.mark.asyncio
    async def test_bad_json(self):
        """Test that bad JSON is not allowed.

        This test ensures that the `log` method raises a `ValidationError` if the
        JSON is not valid.
        """
        system_log = await tvali.SystemLog.new("test")

        with pytest.raises(ValidationError):
            system_log.log(inputs={"foo": {"bar", "baz"}})

    @pytest.mark.asyncio
    async def test_datetime_validator(self):
        """Test datetime validator.

        This test ensures that the `validate_datetime_field` function works correctly.

        The function is expected to raise a `ValueError` if the input is not a valid
        timestamp. It is also expected to return the input timestamp if it is valid.

        The test uses three different inputs to test the function. The first input is
        an invalid timestamp, the second input is a valid timestamp as a string, the
        third input is a valid timestamp as a float, and the fourth input is a valid
        timestamp as a datetime object.
        """
        with pytest.raises(ValueError):
            validate_datetime_field({"not": "a timestamp"})

        test_timestamp = datetime.now()

        assert validate_datetime_field(test_timestamp.isoformat()) == test_timestamp
        assert validate_datetime_field(test_timestamp.timestamp()) == test_timestamp
        assert validate_datetime_field(test_timestamp) == test_timestamp

    @pytest.mark.asyncio
    async def test_uuid_validator(self):
        """Test UUID validator.

        This test ensures that the `validate_uuid` function works correctly.

        The function is expected to raise a `ValueError` if the input is not a valid
        UUID. It is also expected to return the input UUID if it is valid.

        The test uses three different inputs to test the function. The first input is
        an invalid UUID, the second input is a valid UUID as a string, and the third
        input is a valid UUID as a UUID object.
        """
        x = uuid.uuid4()

        assert validate_uuid(x.hex) == x
        assert validate_uuid(x) == x

        with pytest.raises(ValueError):
            validate_uuid("not a uuid")

        with pytest.raises(ValueError):
            validate_uuid(123)

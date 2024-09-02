"""Test Log objects."""
from datetime import datetime
import uuid
import pytest
from pydantic import ValidationError
import tvali
from tvali.log.types import validate_datetime_field, validate_uuid
from .runner import TvaliTestRunner

VALID_IO = {
    'foo': 'bar',
    'baz': 1,
    'qux': [{'foo': 'bar'}],
    'null_value': None
}

class TestLogObjects(TvaliTestRunner):
    @pytest.mark.asyncio
    async def test_instantiation(self):
        system_log = await tvali.SystemLog.new(
            name='MySystemName',
            parameters=VALID_IO
        )

        assert isinstance(system_log, tvali.SystemLog)

        subsystem_log = await system_log.subsystem(
            'MySubsystemName',
            parameters=VALID_IO
        )

        assert isinstance(subsystem_log, tvali.SubsystemLog)

        component_log = await subsystem_log.component(
            name='MyComponentName2'
        )

        assert isinstance(component_log, tvali.ComponentLog)

        subcomponent_log = await component_log.subcomponent(
            name='MySubcomponentName'
        )

        assert isinstance(subcomponent_log, tvali.SubcomponentLog)

    @pytest.mark.asyncio
    async def test_version_enviornment(self):
        system_log = await tvali.SystemLog.new(
            name='MySystemName',
            parameters=VALID_IO
        )

        assert system_log.version == self.VERSION
        assert system_log.environment == self.ENV
            
    @pytest.mark.asyncio
    async def test_event_execution(self):
        system_log = await tvali.SystemLog.new('test')

        with system_log.execute():
            pass
        
        assert isinstance(system_log.start_time, datetime)
        assert isinstance(system_log.end_time, datetime)
        assert system_log.end_time > system_log.start_time
        assert system_log.error_type is None
        assert system_log.error_content is None

    @pytest.mark.asyncio
    async def test_event_error(self):
        system_log = await tvali.SystemLog.new('test')
        with pytest.raises(ValueError):
            with system_log.execute():
                raise ValueError('test')

        assert system_log.error_type == 'ValueError'
        assert isinstance(system_log.error_content, str)

        assert isinstance(system_log.start_time, datetime)
        assert isinstance(system_log.end_time, datetime)
        assert system_log.end_time > system_log.start_time

    @pytest.mark.asyncio
    async def test_event(self):
        system_log = await tvali.SystemLog.new('test')
        assert isinstance(system_log.event(), dict)

    @pytest.mark.asyncio
    async def test_timestamp_constraints(self):
        system_log = await tvali.SystemLog.new('test')
        system_log.start()
        system_log.stop()

        with pytest.raises(ValueError):
            system_log.start()

        with pytest.raises(ValueError):
            system_log.stop()

    @pytest.mark.asyncio
    async def test_duplicate_logs(self):
        system_log = await tvali.SystemLog.new('test')

        for i in ['inputs', 'outputs', 'feedback', 'metadata']:
            system_log.log(**{i: VALID_IO})
            assert system_log.__getattribute__(i) == VALID_IO
            with pytest.raises(ValueError):
                system_log.log(**{i: VALID_IO})

    @pytest.mark.asyncio
    async def test_bad_json(self):
        system_log = await tvali.SystemLog.new('test')
        
        with pytest.raises(ValidationError):
            system_log.log(
                inputs={'foo': {'bar', 'baz'}}
                )

    @pytest.mark.asyncio
    async def test_datetime_validator(self):
        with pytest.raises(ValueError):
            validate_datetime_field({'not': 'a timestamp'})

        test_timestamp = datetime.now()

        assert validate_datetime_field(test_timestamp.isoformat()) == test_timestamp
        assert validate_datetime_field(test_timestamp.timestamp()) == test_timestamp
        assert validate_datetime_field(test_timestamp) == test_timestamp

    @pytest.mark.asyncio
    async def test_uuid_validator(self):
        x = uuid.uuid4()

        assert validate_uuid(x.hex) == x
        assert validate_uuid(x) == x

        with pytest.raises(ValueError):
            validate_uuid('not a uuid')

        with pytest.raises(ValueError):
            validate_uuid(123)

"""Log objects."""
from typing import Any, Dict, Optional, Union
from typing_extensions import Annotated
from uuid import UUID, uuid4
from datetime import datetime
from pydantic import BaseModel, RootModel, Field, BeforeValidator, PlainSerializer

def is_json(data: Any) -> bool:
    """
    Check if data is JSON serializable.

    Checks if data is a simple type (int, float, str, bool, None) or a compound type
    (list, tuple, dict) and all its elements are also JSON serializable. If data is
    a dict, it also checks that all its keys are str.

    :param data: The data to check.
    :return: True if data is JSON serializable, False otherwise.
    """
    if isinstance(data, (int, float, str, bool, type(None))):
        return True
    elif isinstance(data, (list, tuple)):
        return all(is_json(x) for x in data)
    elif isinstance(data, dict):
        return all(isinstance(k, str) and is_json(v) for k, v in data.items())
    else:
        return False

def validate_io_field(obj: dict) -> dict:
    if any(not is_json(x) for x in obj.values()):
        raise ValueError("IO fields must be JSON serializable")

    return obj

def validate_datetime_field(obj: Union[datetime, str, int]) -> datetime:
    if isinstance(obj, str):
        return datetime.fromisoformat(obj)

    if isinstance(obj, int):
        return datetime.fromtimestamp(obj)

    if isinstance(obj, datetime):
        return obj

    raise ValueError("Invalid datetime field")

def serialize_datetime_field(obj: datetime) -> str:
    return obj.isoformat()

IO = Annotated[
    Dict[str, Any],
    BeforeValidator(validate_io_field)
    ]

Metadata = Dict[str, str]


Time = Annotated[
    datetime,
    BeforeValidator(validate_datetime_field),
    PlainSerializer(serialize_datetime_field, return_type=str, when_used='always')
    ]


class Log(BaseModel):
    """Log Base class."""
    id: UUID = Field(default_factory=uuid4)
    name: str
    parameters: Optional[IO] = None
    start_time: Time
    end_time: Time
    error_type: Optional[str] = None
    error_content: Optional[str] = None
    version: str

    inputs: Optional[IO] = None
    outputs: Optional[IO] = None
    feedback: Optional[IO] = None
    metadata: Optional[IO] = None


class SystemLog(Log):
    """System log."""
    ...


class SubsystemLog(Log):
    """Subsystem Log."""
    system_event_id: UUID


class ComponentLog(Log):
    """Component Log."""
    subsystem_event_id: UUID


class SubcomponentLog(Log):
    """Subcomponent Log."""
    component_event_id: UUID

from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import (
    ClassVar as _ClassVar,
    Iterable as _Iterable,
    Mapping as _Mapping,
    Optional as _Optional,
    Union as _Union,
)

DESCRIPTOR: _descriptor.FileDescriptor

class LogType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    UNDECLARED_LOG_TYPE: _ClassVar[LogType]
    EVENT: _ClassVar[LogType]
    RUNTIME: _ClassVar[LogType]
    INPUT: _ClassVar[LogType]
    OUTPUT: _ClassVar[LogType]
    FEEDBACK: _ClassVar[LogType]
    METADATA: _ClassVar[LogType]

class Tier(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    UNDECLARED_TIER: _ClassVar[Tier]
    SYSTEM: _ClassVar[Tier]
    SUBSYSTEM: _ClassVar[Tier]
    COMPONENT: _ClassVar[Tier]
    SUBCOMPONENT: _ClassVar[Tier]

UNDECLARED_LOG_TYPE: LogType
EVENT: LogType
RUNTIME: LogType
INPUT: LogType
OUTPUT: LogType
FEEDBACK: LogType
METADATA: LogType
UNDECLARED_TIER: Tier
SYSTEM: Tier
SUBSYSTEM: Tier
COMPONENT: Tier
SUBCOMPONENT: Tier

class PublishRequest(_message.Message):
    __slots__ = ("records",)
    RECORDS_FIELD_NUMBER: _ClassVar[int]
    records: _containers.RepeatedCompositeFieldContainer[Record]
    def __init__(
        self, records: _Optional[_Iterable[_Union[Record, _Mapping]]] = ...
    ) -> None: ...

class PublishResponse(_message.Message):
    __slots__ = ("successful", "message")
    SUCCESSFUL_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_FIELD_NUMBER: _ClassVar[int]
    successful: bool
    message: str
    def __init__(
        self, successful: bool = ..., message: _Optional[str] = ...
    ) -> None: ...

class Record(_message.Message):
    __slots__ = (
        "tier",
        "log_type",
        "parent_id",
        "id",
        "name",
        "parameters",
        "version",
        "environment",
        "start_time",
        "end_time",
        "error_type",
        "error_content",
        "field_name",
        "field_value",
    )
    TIER_FIELD_NUMBER: _ClassVar[int]
    LOG_TYPE_FIELD_NUMBER: _ClassVar[int]
    PARENT_ID_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    PARAMETERS_FIELD_NUMBER: _ClassVar[int]
    VERSION_FIELD_NUMBER: _ClassVar[int]
    ENVIRONMENT_FIELD_NUMBER: _ClassVar[int]
    START_TIME_FIELD_NUMBER: _ClassVar[int]
    END_TIME_FIELD_NUMBER: _ClassVar[int]
    ERROR_TYPE_FIELD_NUMBER: _ClassVar[int]
    ERROR_CONTENT_FIELD_NUMBER: _ClassVar[int]
    FIELD_NAME_FIELD_NUMBER: _ClassVar[int]
    FIELD_VALUE_FIELD_NUMBER: _ClassVar[int]
    tier: Tier
    log_type: LogType
    parent_id: str
    id: str
    name: str
    parameters: str
    version: str
    environment: str
    start_time: str
    end_time: str
    error_type: str
    error_content: str
    field_name: str
    field_value: str
    def __init__(
        self,
        tier: _Optional[_Union[Tier, str]] = ...,
        log_type: _Optional[_Union[LogType, str]] = ...,
        parent_id: _Optional[str] = ...,
        id: _Optional[str] = ...,
        name: _Optional[str] = ...,
        parameters: _Optional[str] = ...,
        version: _Optional[str] = ...,
        environment: _Optional[str] = ...,
        start_time: _Optional[str] = ...,
        end_time: _Optional[str] = ...,
        error_type: _Optional[str] = ...,
        error_content: _Optional[str] = ...,
        field_name: _Optional[str] = ...,
        field_value: _Optional[str] = ...,
    ) -> None: ...

"""Record base class."""

from typing import Optional, Any, ClassVar, Self, Literal, Annotated
from abc import ABC, abstractmethod
import uuid
from pydantic import BaseModel, Field, create_model, validate_call
from ptolemy.utils import ID, Timestamp, Parameters, LogType, Tier, IOSerializable
from .._core import RecordBuilder, ProtoRecord  # pylint: disable=no-name-in-module


class Record(BaseModel, ABC):
    """Record base class."""

    LOGTYPE: ClassVar[LogType]
    TIER: ClassVar[Tier]

    parent_id: ID
    id: Annotated[ID, Field(default_factory=uuid.uuid4)]

    @classmethod
    @validate_call
    def build(
        cls,
        log_type: LogType,
        tier: Tier,
        parent_id_alias: Literal[
            "always", "on_validation", "on_serialization", "never"
        ] = "on_serialization",
    ) -> type[Self]:
        """Tier."""
        if log_type == LogType.EVENT:
            ltype = Event
        elif log_type == LogType.RUNTIME:
            ltype = Runtime
        elif log_type == LogType.INPUT:
            ltype = Input
        elif log_type == LogType.OUTPUT:
            ltype = Output
        elif log_type == LogType.FEEDBACK:
            ltype = Feedback
        elif log_type == LogType.METADATA:
            ltype = Metadata
        else:
            raise ValueError(f"Unknown log type {log_type}")

        if tier == Tier.SYSTEM:
            if log_type == LogType.EVENT:
                t = "workspace_id"
            else:
                t = f"{tier}_event_id"
        else:
            if log_type == LogType.EVENT:
                t = f"{tier.parent}_event_id"
            else:
                t = f"{tier}_event_id"

        kwargs = {"TIER": (ClassVar[Tier], tier)}

        if parent_id_alias == "on_validation":
            kwargs["parent_id"] = (ID, Field(validation_alias=t))
        if parent_id_alias == "on_serialization":
            kwargs["parent_id"] = (ID, Field(serialization_alias=t))
        if parent_id_alias == "always":
            kwargs["parent_id"] = (ID, Field(alias=t))

        model = create_model(
            f"{log_type.capitalize()}[{tier}]", __base__=ltype, **kwargs
        )

        return model

    @abstractmethod
    def proto(self) -> ProtoRecord:
        """Convert to rust-backend protobuf."""


class Event(Record):
    """Event class."""

    LOGTYPE = LogType.EVENT

    name: str
    parameters: Annotated[Optional[IOSerializable[Parameters]], Field(default=None)]
    environment: Optional[str] = Field(min_length=1, max_length=8, default=None)
    version: Optional[str] = Field(min_length=1, max_length=16, default=None)

    def proto(self) -> ProtoRecord:
        return RecordBuilder().event(
            tier=self.TIER.value,
            parent_id=self.parent_id.hex,
            id=self.id.hex,
            name=self.name,
            parameters=self.parameters.model_dump_json() if self.parameters else None,
            version=self.version,
            environment=self.environment,
        )


class Runtime(Record):
    """Runtime class."""

    LOGTYPE = LogType.RUNTIME

    start_time: Annotated[Optional[Timestamp], Field(default=None)]
    end_time: Annotated[Optional[Timestamp], Field(default=None)]
    error_type: Optional[str] = Field(default=None)
    error_content: Optional[str] = Field(default=None)

    def proto(self) -> ProtoRecord:
        return RecordBuilder().runtime(
            tier=self.TIER.value,
            parent_id=self.parent_id.hex,
            id=self.id.hex,
            start_time=self.start_time.isoformat(),
            end_time=self.end_time.isoformat(),
            error_type=self.error_type,
            error_content=self.error_content,
        )


class _IO(Record):
    """IO base class."""

    field_name: str
    field_value: IOSerializable[Any]

    def proto(self) -> ProtoRecord:
        builder = RecordBuilder()

        if self.LOGTYPE == LogType.INPUT:
            fn = builder.input
        elif self.LOGTYPE == LogType.OUTPUT:
            fn = builder.output
        elif self.LOGTYPE == LogType.FEEDBACK:
            fn = builder.feedback
        else:
            raise ValueError(f"Unexpected log type {self.LOGTYPE}")

        return fn(
            tier=self.TIER.value,
            parent_id=self.parent_id.hex,
            id=self.id.hex,
            field_name=self.field_name,
            field_value=self.field_value.model_dump_json(),
        )


class Input(_IO):
    """Input class."""

    LOGTYPE = LogType.INPUT


class Output(_IO):
    """Output class."""

    LOGTYPE = LogType.OUTPUT


class Feedback(_IO):
    """Feedback class."""

    LOGTYPE = LogType.FEEDBACK


class Metadata(Record):
    """Metadata."""

    LOGTYPE = LogType.METADATA

    field_name: str
    field_value: str

    def proto(self) -> ProtoRecord:
        return RecordBuilder().metadata(
            tier=self.TIER.value,
            parent_id=self.parent_id.hex,
            id=self.id.hex,
            field_name=self.field_name,
            field_value=self.field_value,
        )


RECORD_MAP = {
    tier: {log_type: Record.build(log_type, tier) for log_type in LogType}
    for tier in Tier
}


def get_record_type(log_type: LogType, tier: Tier) -> type[Record]:
    """Get record type."""
    return RECORD_MAP[tier][log_type]

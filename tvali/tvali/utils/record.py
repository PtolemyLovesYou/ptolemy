"""Record base class."""

from typing import Optional, Generic, TypeVar, Any, ClassVar, Self
from pydantic import BaseModel, Field, create_model
import uuid
from tvali.utils import ID, Timestamp, Parameters, LogType, Tier

T = TypeVar("T")


class Record(BaseModel):
    """Record base class."""

    LOGTYPE: ClassVar[LogType]
    TIER: ClassVar[Tier]

    parent_id: ID
    id: ID = Field(default_factory=uuid.uuid4)

    @classmethod
    def tier(cls, tier: Tier) -> type[Self]:
        """Tier."""
        t = tier.parent or "workspace" if tier == Tier.SYSTEM else tier

        model = create_model(
            f"{cls.LOGTYPE.capitalize()}[{tier}]",
            __base__=cls,
            TIER=(ClassVar[Tier], tier),
            parent_id=(ID, Field(serialization_alias=f"{t}_event_id")),
        )

        return model


class Event(Record):
    """Event class."""

    LOGTYPE = LogType.EVENT

    name: str
    parameters: Optional[Parameters] = Field(default=None)
    environment: Optional[str] = Field(min_length=1, max_length=8, default=None)
    version: Optional[str] = Field(min_length=1, max_length=16, default=None)

    def spawn(
        self,
        name: str,
        parameters: Optional[Parameters] = None,
        environment: Optional[str] = None,
        version: Optional[str] = None,
    ) -> "Event":
        """Spawn a new event as a child of this event."""
        if self.TIER.child is None:
            raise ValueError(f"Cannot spawn child of tier {self.TIER}")

        return self.tier(self.TIER.child)(
            parent_id=self.id,
            name=name,
            parameters=parameters,
            environment=environment,
            version=version,
        )


class Runtime(Record):
    """Runtime class."""

    LOGTYPE = LogType.RUNTIME

    start_time: Timestamp
    end_time: Timestamp
    error_type: Optional[str] = Field(default=None)
    error_content: Optional[str] = Field(default=None)


class _IO(Record, Generic[T]):
    """IO base class."""

    field_name: str
    field_value: T


class Input(_IO[Any]):
    """Input class."""

    LOGTYPE = LogType.INPUT


class Output(_IO[Any]):
    """Output class."""

    LOGTYPE = LogType.OUTPUT


class Feedback(_IO[Any]):
    """Feedback class."""

    LOGTYPE = LogType.FEEDBACK


class Metadata(_IO[str]):
    """Metadata class."""

    LOGTYPE = LogType.METADATA


def get_record_class(log_type: LogType, tier: Tier) -> type[Record]:
    """
    Get the Record class for the given log type and tier.

    Args:
        log_type: Type of log.
        tier: Tier of log.

    Returns:
        type[Record]: Record class for the given log type and tier.
    """
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

    return ltype.tier(tier)

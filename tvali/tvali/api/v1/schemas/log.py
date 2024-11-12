"""Log schema factory."""

from typing import Generic, Optional, Dict, Any, ClassVar
from pydantic import BaseModel, create_model, Field
from ....utils.enums import Tier, LogType
from ....utils.types import RequiredID, Timestamp, T


# Log mixins
class LogMixin(BaseModel):
    """Log mixin."""


class EventLogMixin(LogMixin):
    """Event mixin."""

    name: str
    parameters: Dict[str, Any]
    environment: str = Field(min_length=1, max_length=8)
    version: str = Field(min_length=1, max_length=16)


class RuntimeLogMixin(LogMixin):
    """Runtime mixin."""

    start_time: Timestamp
    end_time: Timestamp
    error_type: Optional[str] = None
    error_content: Optional[str] = None


class IOLogMixin(BaseModel, Generic[T]):
    """IO mixin."""

    field_name: str
    field_value: T


LOG_MIXIN_MAP = {
    LogType.EVENT: EventLogMixin,
    LogType.RUNTIME: RuntimeLogMixin,
    LogType.INPUT: IOLogMixin[Any],
    LogType.OUTPUT: IOLogMixin[Any],
    LogType.FEEDBACK: IOLogMixin[Any],
    LogType.METADATA: IOLogMixin[str],
}


# Schema mixins
class Base(BaseModel):
    """Base schema."""

    NAME: ClassVar[str] = "Base"


class Create(Base):
    """Create schema."""

    NAME: ClassVar[str] = "Create"


class Record(Base):
    """Record schema."""

    NAME: ClassVar[str] = "Record"

    id: RequiredID


def dependent_mixin(tier: Tier, log_type: LogType) -> dict[str, tuple[type, Field]]:
    """
    Return a dictionary with the dependent fields to be included in the schema
    based on the given tier and log type.

    If the tier is Tier.SYSTEM and the log type is LogType.EVENT, an empty
    dictionary is returned.

    Otherwise, a dictionary containing a single key-value pair is returned.
    The key is the name of the foreign key field (e.g. system_event_id,
    subsystem_event_id, etc.) and the value is a tuple of the type of the
    field (RequiredID) and a Field object.

    Args:
        tier: The tier of the schema (e.g. SYSTEM, SUBSYSTEM, COMPONENT, SUBCOMPONENT).
        log_type: The type of the log (e.g. EVENT, RUNTIME, INPUT, OUTPUT, FEEDBACK, METADATA).

    Returns:
        A dictionary with the dependent fields for the schema.

    Raises:
        ValueError: If the tier is unknown.
    """
    if tier == Tier.SYSTEM:
        if log_type == LogType.EVENT:
            return {}

        parent = Tier.SYSTEM
    elif tier == Tier.SUBSYSTEM:
        parent = Tier.SYSTEM if log_type == LogType.EVENT else Tier.SUBSYSTEM
    elif tier == Tier.COMPONENT:
        parent = Tier.SUBSYSTEM if log_type == LogType.EVENT else Tier.COMPONENT
    elif tier == Tier.SUBCOMPONENT:
        parent = Tier.COMPONENT if log_type == LogType.EVENT else Tier.SUBCOMPONENT
    else:
        raise ValueError(f"Unknown tier: {tier}")

    return {
        f"{parent}_event_id": (RequiredID, Field()),
    }


class LogMetaclass(type):
    """Metaclass for LogSchema class."""

    def __getitem__(cls, mixins: tuple[Tier, LogType, Base]) -> type[BaseModel]:
        name = f"{mixins[0].capitalize()}{mixins[1].capitalize()}{mixins[2].NAME}"
        return create_model(
            name,
            **dependent_mixin(mixins[0], mixins[1]),
            __base__=(LOG_MIXIN_MAP[mixins[1]], mixins[2]),
        )


class LogSchema(metaclass=LogMetaclass):  # pylint: disable=too-few-public-methods
    """Log schema."""

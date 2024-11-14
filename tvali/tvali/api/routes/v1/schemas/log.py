"""Log schema factory."""

from typing import Generic, Optional, Dict, Any, ClassVar, TypeVar
from pydantic import BaseModel, create_model, Field, ConfigDict
from .....utils import Tier, LogType, ID, Timestamp

T = TypeVar("T")


class Mixin(BaseModel):
    """Mixin."""


MixinType_co = TypeVar(  # pylint: disable=invalid-name
    "MixinType_co", bound=Mixin, covariant=True
)


# Log mixins
class LogMixin(Mixin):
    """Log mixin."""


class EventLogMixin(LogMixin):
    """Event mixin."""

    name: str
    parameters: Optional[Dict[str, Any]] = None
    environment: Optional[str] = Field(min_length=1, max_length=8, default=None)
    version: Optional[str] = Field(min_length=1, max_length=16, default=None)


class RuntimeLogMixin(LogMixin):
    """Runtime mixin."""

    start_time: Timestamp
    end_time: Timestamp
    error_type: Optional[str] = None
    error_content: Optional[str] = None


class IOLogMixin(Mixin, Generic[T]):
    """IO mixin."""

    field_name: str
    field_value: T


# Query mixins
class QueryMixin(Mixin):
    """Query Mixin."""

    model_config = ConfigDict(extra="forbid")

    id: Optional[ID] = None

    limit: int = Field(default=10, ge=1, le=100)
    offset: int = Field(default=0, ge=0)


class EventQueryMixin(QueryMixin):
    """Event Query Mixin."""

    name: Optional[str] = None
    environment: Optional[str] = None
    version: Optional[str] = None


class RuntimeQueryMixin(QueryMixin):
    """Runtime Query mixin."""

    error_type: Optional[str] = None


class IOLogQueryMixin(QueryMixin):
    """IOLog Query Mixin."""

    field_name: Optional[str] = None


LOG_MIXIN_MAP = {
    LogType.EVENT: EventLogMixin,
    LogType.RUNTIME: RuntimeLogMixin,
    LogType.INPUT: IOLogMixin[Any],
    LogType.OUTPUT: IOLogMixin[Any],
    LogType.FEEDBACK: IOLogMixin[Any],
    LogType.METADATA: IOLogMixin[str],
}

QUERY_MIXIN_MAP = {
    LogType.EVENT: EventQueryMixin,
    LogType.RUNTIME: RuntimeQueryMixin,
    LogType.INPUT: IOLogQueryMixin,
    LogType.OUTPUT: IOLogQueryMixin,
    LogType.FEEDBACK: IOLogQueryMixin,
    LogType.METADATA: IOLogQueryMixin,
}


# Schema mixins
class BaseSchema(Mixin):
    """Base schema."""

    NAME: ClassVar[str] = "Base"


class CreateSchema(BaseSchema):
    """Create schema."""

    NAME: ClassVar[str] = "Create"

    id: Optional[ID] = None


class RecordSchema(BaseSchema):
    """Record schema."""

    NAME: ClassVar[str] = "Record"

    id: ID


def dependent_mixin(
    tier: Tier, log_type: LogType, optional: bool = False
) -> dict[str, tuple[type, Field]]:
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

    if optional:
        return {
            f"{parent}_event_id": (Optional[ID], Field(default=None)),
        }

    return {
        f"{parent}_event_id": (ID, Field()),
    }


class LogMetaclass(type):
    """Metaclass for LogSchema class."""

    def __getitem__(
        cls, mixins: tuple[Tier, LogType, type[MixinType_co]]
    ) -> type[MixinType_co]:
        if issubclass(mixins[2], QueryMixin):
            return create_model(
                f"{mixins[0].capitalize()}{mixins[1].capitalize()}Query",
                **dependent_mixin(mixins[0], mixins[1], optional=True),
                __base__=(QUERY_MIXIN_MAP[mixins[1]]),
            )

        name = f"{mixins[0].capitalize()}{mixins[1].capitalize()}{mixins[2].NAME}"
        return create_model(
            name,
            **dependent_mixin(mixins[0], mixins[1], optional=False),
            __base__=(LOG_MIXIN_MAP[mixins[1]], mixins[2]),
        )


class Log(metaclass=LogMetaclass):  # pylint: disable=too-few-public-methods
    """Log schema."""

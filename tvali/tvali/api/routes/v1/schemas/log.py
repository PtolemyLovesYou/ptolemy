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


class LogMetaclass(type):
    """Metaclass for LogSchema class."""

    LOG_MIXIN_MAP: Dict[LogType, type[LogMixin]] = {
        LogType.EVENT: EventLogMixin,
        LogType.RUNTIME: RuntimeLogMixin,
        LogType.INPUT: IOLogMixin[Any],
        LogType.OUTPUT: IOLogMixin[Any],
        LogType.FEEDBACK: IOLogMixin[Any],
        LogType.METADATA: IOLogMixin[str],
    }

    QUERY_MIXIN_MAP: Dict[LogType, type[QueryMixin]] = {
        LogType.EVENT: EventQueryMixin,
        LogType.RUNTIME: RuntimeQueryMixin,
        LogType.INPUT: IOLogQueryMixin,
        LogType.OUTPUT: IOLogQueryMixin,
        LogType.FEEDBACK: IOLogQueryMixin,
        LogType.METADATA: IOLogQueryMixin,
    }

    def __getitem__(
        cls, mixins: tuple[Tier, LogType, type[MixinType_co]]
    ) -> type[MixinType_co]:
        # figure out dependency fields.
        parent = mixins[0].parent if mixins[1] == LogType.EVENT else mixins[0]
        dependent_mixin_fields = {}
        name_suffix = "Query"

        if issubclass(mixins[2], QueryMixin):
            name_suffix = "Query"

            if parent is not None:
                dependent_mixin_fields[f"{parent}_event_id"] = (
                    Optional[ID],
                    Field(default=None),
                )

            base_class = cls.QUERY_MIXIN_MAP[mixins[1]]
        else:
            name_suffix = mixins[2].NAME

            if parent is not None:
                dependent_mixin_fields[f"{parent}_event_id"] = (ID, Field())

            base_class = (cls.LOG_MIXIN_MAP[mixins[1]], mixins[2])

        return create_model(
            f"{mixins[0].capitalize()}{mixins[1].capitalize()}{name_suffix}",
            **dependent_mixin_fields,
            __base__=base_class,
        )


class Log(metaclass=LogMetaclass):  # pylint: disable=too-few-public-methods
    """Log schema."""

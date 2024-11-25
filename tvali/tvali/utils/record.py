"""Record base class."""

from typing import Optional, Any, ClassVar, Self, Literal
import uuid
from pydantic import BaseModel, Field, create_model, validate_call
from tvali.utils import ID, Timestamp, Parameters, LogType, Tier, IOSerializable
from ..proto import observer_pb2 as observer


class Record(BaseModel):
    """Record base class."""

    LOGTYPE: ClassVar[LogType]
    TIER: ClassVar[Tier]

    parent_id: ID
    id: ID = Field(default_factory=uuid.uuid4)

    def proto_dict(self) -> dict:
        """
        Convert the record instance to a dictionary representation suitable for proto serialization.

        Returns:
            dict: A dictionary containing the fields of the record formatted for proto.
        """
        raise NotImplementedError("Method not implemented!")

    @classmethod
    def proto_args(cls, record: observer.Record) -> dict:  # pylint: disable=no-member
        """
        Get a dictionary of arguments to pass to the proto Record constructor
        from a proto Record.

        Args:
            record (observer.Record): The proto record to extract arguments from.

        Returns:
            dict: A dictionary of arguments to pass to the proto Record constructor.
        """
        raise NotImplementedError("Method not implemented!")

    def proto(self) -> observer.Record:  # pylint: disable=no-member
        """
        Get the proto Record for this record.

        Returns:
            observer.Record: The proto record.
        """
        return observer.Record(  # pylint: disable=no-member
            tier=self.TIER.proto(),
            log_type=self.LOGTYPE.proto(),
            **self.model_dump(include=["id", "parent_id"]),
            **self.proto_dict(),
        )

    @staticmethod
    def from_proto(proto: observer.Record) -> Self:  # pylint: disable=no-member
        """
        Convert a proto Record to a Record instance.

        Args:
            proto (observer.Record): The proto record to convert.

        Returns:
            Record: An instance of the Record class corresponding to the proto record.
        """
        built_cls = Record.build(
            LogType.from_proto(proto.log_type),
            Tier.from_proto(proto.tier),
        )

        return built_cls(
            parent_id=proto.parent_id,
            id=proto.id,
            **built_cls.proto_args(proto),
        )

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


class Event(Record):
    """Event class."""

    LOGTYPE = LogType.EVENT

    name: str
    parameters: Optional[IOSerializable[Parameters]] = Field(default=None)
    environment: Optional[str] = Field(min_length=1, max_length=8, default=None)
    version: Optional[str] = Field(min_length=1, max_length=16, default=None)

    def proto_dict(self) -> dict:
        data = self.model_dump(
            include=["name", "environment", "version"], exclude_none=True
        )

        if self.parameters is not None:
            data["parameters"] = (
                self.parameters.model_dump_json()  # pylint: disable=no-member
            )

        return data

    @classmethod
    def proto_args(cls, record: observer.Record) -> dict:  # pylint: disable=no-member
        return {
            "name": record.name,
            "parameters": (
                IOSerializable[Parameters].validate_json(record.parameters)
                if record.parameters
                else None
            ),
            "environment": record.environment,
            "version": record.version,
        }

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

        return Record.build(LogType.EVENT, self.TIER.child)(
            parent_id=self.id,
            name=name,
            parameters=parameters,
            environment=environment,
            version=version,
        )


class Runtime(Record):
    """Runtime class."""

    LOGTYPE = LogType.RUNTIME

    start_time: Optional[Timestamp] = Field(default=None)
    end_time: Optional[Timestamp] = Field(default=None)
    error_type: Optional[str] = Field(default=None)
    error_content: Optional[str] = Field(default=None)

    @classmethod
    def proto_args(cls, record: observer.Record) -> dict:  # pylint: disable=no-member
        return {
            "start_time": record.start_time,
            "end_time": record.end_time,
            "error_type": record.error_type,
            "error_content": record.error_content,
        }

    def proto_dict(self) -> dict:
        return self.model_dump(
            exclude_none=True,
            include=["start_time", "end_time", "error_type", "error_content"],
        )


class _IO(Record):
    """IO base class."""

    field_name: str
    field_value: IOSerializable[Any]

    @classmethod
    def proto_args(cls, record: observer.Record) -> dict:  # pylint: disable=no-member
        return {
            "field_name": record.field_name,
            "field_value": IOSerializable[Any].validate_json(record.field_value),
        }

    def proto_dict(self) -> dict:
        return {
            "field_name": self.field_name,
            "field_value": self.field_value.model_dump_json(),
        }


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

    @classmethod
    def proto_args(cls, record: observer.Record) -> dict:  # pylint: disable=no-member
        return {
            "field_name": record.field_name,
            "field_value": record.field_value,
        }

    def proto_dict(self) -> dict:
        return self.model_dump()

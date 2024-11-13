"""Client."""

from typing import Any, Dict, Optional, ClassVar, Union
from enum import StrEnum
from abc import abstractmethod, ABC
from datetime import datetime
import traceback
from contextlib import asynccontextmanager
import uuid
from pydantic import (
    PrivateAttr,
    computed_field,
    Field,
    create_model,
    BaseModel,
    RootModel,
    field_validator,
    field_serializer
    )


class Timestamp(RootModel):
    """Timestamp type."""
    root: datetime = Field()

    @classmethod
    def now(cls) -> "Timestamp":
        """Get current time. Wraps datetime.now()."""
        return Timestamp(datetime.now())

    @field_validator("root")
    @classmethod
    def validate_timestamp(cls, v: Union[str, datetime]) -> datetime:
        """
        Validate a timestamp.

        Args:
            v: Value to validate.

        Returns:
            datetime: Validated timestamp.

        Raises:
            ValueError: If value is not a valid timestamp.
        """
        if isinstance(v, datetime):
            return v

        if isinstance(v, str):
            return datetime.fromisoformat(v)

        raise ValueError(f"Invalid timestamp: {v}")

    @field_serializer('root', when_used="always")
    def serialize_timestamp(self, v: datetime) -> str:
        """
        Serialize timestamp to ISO format.

        Args:
            v: Timestamp to serialize.

        Returns:
            str: ISO format timestamp.
        """
        return v.isoformat()

class ID(RootModel):
    """ID type."""
    root: Optional[uuid.UUID] = Field()

    @classmethod
    def new(self) -> "ID":
        return ID(uuid.uuid4())

    @field_validator("root")
    @classmethod
    def validate_uuid(cls, v: Union[str, uuid.UUID]) -> uuid.UUID:
        """
        Validate a UUID.

        Args:
            v: Value to validate.

        Returns:
            UUID: Validated UUID.

        Raises:
            ValueError: If value is not a valid UUID.
        """
        if isinstance(v, str):
            return uuid.UUID(v)

        if isinstance(v, uuid.UUID):
            return v

        raise ValueError(f"Invalid UUID: {v}")

    @field_serializer('root', when_used="always")
    def serialize_uuid(self, v: uuid.UUID) -> str:
        """
        Serialize UUID to hex string.

        Args:
            v: UUID to serialize.

        Returns:
            str: Hex string representation of UUID.
        """
        return v.hex

class Tier(StrEnum):
    """Tiers."""

    SYSTEM = "system"
    SUBSYSTEM = "subsystem"
    COMPONENT = "component"
    SUBCOMPONENT = "subcomponent"

    @property
    def parent(self) -> Optional["Tier"]:
        """Get parent tier."""
        return {
            Tier.SUBCOMPONENT: Tier.COMPONENT,
            Tier.COMPONENT: Tier.SUBSYSTEM,
            Tier.SUBSYSTEM: Tier.SYSTEM,
        }.get(self)

    @property
    def child(self) -> Optional["Tier"]:
        """Get child tier."""
        return {
            Tier.SYSTEM: Tier.SUBSYSTEM,
            Tier.SUBSYSTEM: Tier.COMPONENT,
            Tier.COMPONENT: Tier.SUBCOMPONENT,
        }.get(self)


Parameters = Dict[str, Any]
IO = Dict[str, Any]
Metadata = Dict[str, str]


class Runtime(BaseModel):
    """Runtime model."""

    _start_time: Timestamp = PrivateAttr(default=None)
    _end_time: Timestamp = PrivateAttr(default=None)
    _error_type: Optional[str] = PrivateAttr(default=None)
    _error_content: Optional[str] = PrivateAttr(default=None)

    @computed_field
    @property
    def start_time(self) -> Timestamp:
        """Start time."""
        return self._start_time

    @computed_field
    @property
    def end_time(self) -> Timestamp:
        """End time."""
        return self._end_time

    @computed_field
    @property
    def error_type(self) -> Optional[str]:
        """Error type."""
        return self._error_type

    @computed_field
    @property
    def error_content(self) -> Optional[str]:
        """Error content."""
        return self._error_content

    def start(self):
        """
        Start runtime.

        Raises:
            ValueError: If runtime already started or ended.
        """
        if self._start_time:
            raise ValueError("Runtime already started")

        if self._end_time:
            raise ValueError("Runtime already ended")

        self._start_time = Timestamp.now()

    def end(self):
        """
        End runtime.

        Raises:
            ValueError: If runtime not started.
            ValueError: If runtime already ended.
        """
        if not self._start_time:
            raise ValueError("Runtime not started")

        if self._end_time:
            raise ValueError("Runtime already ended")

        self._end_time = Timestamp.now()

    def log_error(self, error_type: str, error_content: str):
        """
        Log error.

        Args:
            error_type (str): Error type.
            error_content (str): Error content.

        Raises:
            ValueError: If runtime already has an error.
        """
        if self._error_type or self._error_content:
            raise ValueError("Runtime already has an error")

        self._error_type = error_type
        self._error_content = error_content


class Log(BaseModel, ABC):
    """Log base class."""

    _TIER: ClassVar[Tier]

    _runtime: Runtime = PrivateAttr(default_factory=Runtime)
    _inputs: IO = PrivateAttr(default=None)
    _outputs: IO = PrivateAttr(default=None)
    _feedback: IO = PrivateAttr(default=None)
    _metadata: Metadata = PrivateAttr(default=None)

    id: ID = Field(default_factory=ID.new)
    name: str = Field()
    parameters: Optional[Parameters] = Field(default=None)
    version: Optional[str] = Field(min_length=1, max_length=16, default=None)
    environment: Optional[str] = Field(min_length=1, max_length=8, default=None)

    @classmethod
    def tier(cls, tier: Tier) -> type["Log"]:
        """
        Create a new log type with the given tier.

        Args:
            tier (Tier): Tier of the new log type.

        Returns:
            type[Log]: New log type with the given tier.
        """
        name = f"{cls.__name__}[{tier.capitalize()}]"

        fields = {}

        if tier.parent:
            fields[f"{tier.parent}_event_id"] = (uuid.UUID, Field())

        model = create_model(
            name,
            __base__=cls,
            **fields,
        )

        if tier.child:
            setattr(model, "_TIER", tier)

        return model

    def id_dict(self) -> Dict[str, str]:
        """
        Convert the log ID to a dictionary format.

        Returns:
            Dict[str, str]: A dictionary with the log's tier-specific event ID
            as the key and the serialized ID as the value.
        """
        return {f"{self._TIER}_event_id": self.id.model_dump()} # pylint: disable=no-member

    @computed_field
    @property
    def runtime(self) -> Runtime:
        """Get log runtime."""
        return self._runtime

    @computed_field
    @property
    def inputs(self) -> IO | None:
        """Get log inputs."""
        return self._inputs

    @computed_field
    @property
    def outputs(self) -> IO | None:
        """Get log outputs."""
        return self._outputs

    @computed_field
    @property
    def feedback(self) -> IO | None:
        """Get log feedback."""
        return self._feedback

    @computed_field
    @property
    def metadata(self) -> Metadata | None:
        """Get log metadata."""
        return self._metadata

    async def log(
        self,
        inputs: Optional[IO] = None,
        outputs: Optional[IO] = None,
        feedback: Optional[IO] = None,
        metadata: Optional[Metadata] = None,
    ):
        """
        Log a single event.

        Args:
        inputs: Event inputs
        outputs: Event outputs
        feedback: Event feedback
        metadata: Event metadata

        Raises:
        ValueError: If any of the parameters are already set
        """
        if inputs is not None:
            if self._inputs is not None:
                raise ValueError("Inputs already set")
            self._inputs = inputs

        if outputs is not None:
            if self._outputs is not None:
                raise ValueError("Outputs already set")
            self._outputs = outputs

        if feedback is not None:
            if self._feedback is not None:
                raise ValueError("Feedback already set")
            self._feedback = feedback

        if metadata is not None:
            if self._metadata is not None:
                raise ValueError("Metadata already set")
            self._metadata = metadata

    @abstractmethod
    async def push(self) -> None:
        """Push log."""

    @abstractmethod
    async def delete(self, id_: str) -> None:
        """Delete log."""

    @asynccontextmanager
    async def observe(self):
        """
        Asynchronous context manager that logs the execution time and any
        exceptions that occur in the block.

        The log will be pushed to the server automatically when the block
        is exited.

        Example:
        >>> async with client.observe():
        ...     # Do something
        """
        self._runtime.start()
        try:
            yield
        except Exception as e:
            self._runtime.log_error(e.__class__.__name__, traceback.format_exc())
            raise e
        finally:
            self._runtime.end()
            await self.push()

    async def spawn(
        self,
        name: str,
        parameters: Optional[Parameters] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> "Log":
        """
        Spawn a new log as a child of this log.

        Args:
            name (str): Name of the new log.
            parameters (Optional[Parameters], optional): Parameters of the new log.

        Returns:
            Log: The new log.
        """
        if self._TIER.child is None:
            raise ValueError(f"Cannot spawn child of tier {self._TIER}")

        id_kwargs = {f"{self._TIER}_event_id": self.id}

        return self.tier(self._TIER.child)(
            name=name,
            parameters=parameters,
            **id_kwargs,
            version=version,
            environment=environment,
        )

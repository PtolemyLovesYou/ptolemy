"""Log."""

from typing import Dict, Optional, ClassVar, Any, List, Generic, TypeVar
import uuid
from datetime import datetime
from abc import abstractmethod, ABC
import traceback
from contextlib import asynccontextmanager
from pydantic import (
    Field,
    create_model,
    BaseModel,
    ConfigDict,
)
from ..config import TransportConfig
from ...utils import ID, Timestamp, Parameters, Tier

T = TypeVar("T")


class IORecord(BaseModel, Generic[T]):
    """IO record."""

    id: ID = Field(default_factory=uuid.uuid4)
    field_name: str
    field_value: T

    @classmethod
    def tier(cls, tier: Tier) -> type["IORecord"]:
        """Get IORecord type with the given tier."""
        return create_model(
            f"{cls.__name__}[{tier.capitalize()}]",
            __base__=cls,
            **{
                f"{tier}_event_id": (ID, Field()),
            },
        )


class RuntimeMixin(BaseModel):
    """Runtime model."""

    model_config = ConfigDict(validate_assignment=True)

    start_time: Timestamp = Field(default=None, init=False)
    end_time: Timestamp = Field(default=None, init=False)
    error_type: Optional[str] = Field(default=None, init=False)
    error_content: Optional[str] = Field(default=None, init=False)

    @property
    def completed(self) -> bool:
        """Whether runtime is completed."""
        if self.start_time is None and self.end_time is not None:
            raise ValueError("Runtime not started")

        return self.start_time is not None and self.end_time is not None

    def start(self):
        """
        Start runtime.

        Raises:
            ValueError: If runtime already started or ended.
        """
        if self.start_time:
            raise ValueError("Runtime already started")

        if self.end_time:
            raise ValueError("Runtime already ended")

        self.start_time = datetime.now()

    def end(self):
        """
        End runtime.

        Raises:
            ValueError: If runtime not started.
            ValueError: If runtime already ended.
        """
        if not self.start_time:
            raise ValueError("Runtime not started")

        if self.end_time:
            raise ValueError("Runtime already ended")

        self.end_time = datetime.now()

    def log_error(self, error_type: str, error_content: str):
        """
        Log error.

        Args:
            error_type (str): Error type.
            error_content (str): Error content.

        Raises:
            ValueError: If runtime already has an error.
        """
        if self.error_type or self.error_content:
            raise ValueError("Runtime already has an error")

        self.error_type = error_type
        self.error_content = error_content


class EventRecordMixin(BaseModel):
    """Event record mixin."""

    id: ID = Field(default_factory=uuid.uuid4)
    name: str = Field()
    parameters: Optional[Parameters] = Field(default=None)
    version: Optional[str] = Field(min_length=1, max_length=16, default=None)
    environment: Optional[str] = Field(min_length=1, max_length=8, default=None)


class IOMixin(BaseModel):
    """IO mixin."""

    inputs: Optional[List[IORecord[Any]]] = Field(default_factory=list)
    outputs: Optional[List[IORecord[Any]]] = Field(default_factory=list)
    feedback: Optional[List[IORecord[Any]]] = Field(default_factory=list)
    metadata: Optional[List[IORecord[str]]] = Field(default_factory=list)


class Log(EventRecordMixin, IOMixin, RuntimeMixin, ABC):
    """Log base class."""

    model_config = ConfigDict(validate_assignment=True)

    TRANSPORT_CONFIG: ClassVar[TransportConfig]
    LOG_CLS: ClassVar[type["Log"]]
    TIER: ClassVar[Tier]

    @classmethod
    def configure(
        cls, tier: Tier, log_cls: type["Log"], transport_config: TransportConfig
    ) -> type["Log"]:
        """
        Create a new log type with the given tier.

        Args:
            tier (Tier): Tier of the new log type.

        Returns:
            type[Log]: New log type with the given tier.
        """
        name = f"Log[{tier.capitalize()}]"

        fields = {
            "TRANSPORT_CONFIG": (ClassVar[TransportConfig], transport_config),
            "LOG_CLS": (ClassVar[type[Log]], log_cls),
            "TIER": (ClassVar[Tier], tier),
        }

        if tier.parent:
            fields[f"{tier.parent}_event_id"] = (ID, Field())

        model = create_model(
            name,
            __base__=log_cls,
            **fields,
        )

        return model

    def id_dict(self) -> Dict[str, str]:
        """
        Convert the log ID to a dictionary format.

        Returns:
            Dict[str, str]: A dictionary with the log's tier-specific event ID
            as the key and the serialized ID as the value.
        """
        return {
            f"{self.TIER}_event_id": self.id.hex  # pylint: disable=no-member
        }

    def event_dict(self) -> dict:
        """Get event dict."""
        return self.model_dump(
            exclude=["runtime", "inputs", "outputs", "feedback", "metadata"],
            exclude_none=True,
        )

    def runtime_dict(self) -> dict:
        """Get log runtime."""
        return (
            self.model_dump(
                include=[*RuntimeMixin.model_fields.keys()], exclude_none=True
            )  # pylint: disable=no-member
            | self.id_dict()
        )

    def inputs_dicts(self) -> List[Dict[str, Any]] | None:
        """Get log inputs."""
        return [i.model_dump() for i in self.inputs] or None

    def outputs_dicts(self) -> List[Dict[str, Any]] | None:
        """Get log outputs."""
        return [o.model_dump() for o in self.outputs] or None

    def feedback_dicts(self) -> List[Dict[str, Any]] | None:
        """Get log feedback."""
        return [f.model_dump() for f in self.feedback] or None

    def metadata_dicts(self) -> List[Dict[str, str]] | None:
        """Get log metadata."""
        return [m.model_dump() for m in self.metadata] or None

    async def log(
        self,
        inputs: Optional[Dict[str, Any]] = None,
        outputs: Optional[Dict[str, Any]] = None,
        feedback: Optional[Dict[str, Any]] = None,
        metadata: Optional[Dict[str, Any]] = None,
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
            for field_name, field_value in inputs.items():
                if field_name in self.inputs:
                    raise ValueError(f"Input {field_name} already set")

                self.inputs.append(  # pylint: disable=no-member
                    IORecord[Any].tier(self.TIER)(
                        field_name=field_name, field_value=field_value, **self.id_dict()
                    )
                )

        if outputs is not None:
            for field_name, field_value in outputs.items():
                if field_name in self.outputs:
                    raise ValueError(f"Output {field_name} already set")

                self.outputs.append(  # pylint: disable=no-member
                    IORecord[Any].tier(self.TIER)(
                        field_name=field_name, field_value=field_value, **self.id_dict()
                    )
                )

        if feedback is not None:
            for field_name, field_value in feedback.items():
                if field_name in self.feedback:
                    raise ValueError(f"Feedback {field_name} already set")

                self.feedback.append(  # pylint: disable=no-member
                    IORecord[Any].tier(self.TIER)(
                        field_name=field_name, field_value=field_value, **self.id_dict()
                    )
                )

        if metadata is not None:
            for field_name, field_value in metadata.items():
                if field_name in self.metadata:
                    raise ValueError(f"Metadata {field_name} already set")

                self.metadata.append(  # pylint: disable=no-member
                    IORecord[str].tier(self.TIER)(
                        field_name=field_name, field_value=field_value, **self.id_dict()
                    )
                )

    @abstractmethod
    async def push_on_beginning(self) -> None:
        """Push log."""

    @abstractmethod
    async def push_on_ending(self) -> None:
        """Push log."""

    @abstractmethod
    async def delete(self) -> None:
        """Delete log."""

    @asynccontextmanager
    async def observe(self, time: bool = True):
        """
        Asynchronous context manager that logs the execution time and any
        exceptions that occur in the block.

        The log will be pushed to the server automatically when the block
        is exited.

        Example:
        >>> async with client.observe():
        ...     # Do something
        """
        await self.push_on_beginning()

        if time:
            self.start()
        try:
            yield
        except Exception as e:
            self.log_error(  # pylint: disable=no-member
                e.__class__.__name__, traceback.format_exc()
            )
            raise e
        finally:
            if time:
                self.end()

            if time and not self.completed:  # pylint: disable=no-member
                raise RuntimeError(
                    "Runtime isn't completed. Make sure to call .start() and .end() inside your .observe() clause."
                )

            await self.push_on_ending()

    @classmethod
    def new(
        cls,
        name: str,
        parameters: Optional[Parameters] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> "Log":
        return cls(
            name=name, parameters=parameters, version=version, environment=environment
        )

    def spawn(
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
        if self.TIER.child is None:
            raise ValueError(f"Cannot spawn child of tier {self.TIER}")

        id_kwargs = {f"{self.TIER}_event_id": self.id}

        return self.configure(self.TIER.child, self.LOG_CLS, self.TRANSPORT_CONFIG)(
            **id_kwargs,
            name=name,
            parameters=parameters,
            version=version,
            environment=environment,
        )

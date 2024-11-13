"""Client."""

from typing import Dict, Optional, ClassVar, Any
from abc import abstractmethod, ABC
import traceback
from contextlib import asynccontextmanager
import uuid
from pydantic import (
    PrivateAttr,
    computed_field,
    Field,
    create_model,
    BaseModel,
)
from .utils.types import ID, Timestamp, Parameters, IO
from .utils.enums import Tier


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
    _inputs: IO[Any] = PrivateAttr(default=None)
    _outputs: IO[Any] = PrivateAttr(default=None)
    _feedback: IO[Any] = PrivateAttr(default=None)
    _metadata: IO[str] = PrivateAttr(default=None)

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
        return {
            f"{self._TIER}_event_id": self.id.model_dump()
        }  # pylint: disable=no-member

    @computed_field
    @property
    def runtime(self) -> Runtime:
        """Get log runtime."""
        return self._runtime

    @computed_field
    @property
    def inputs(self) -> Dict[str, Any] | None:
        """Get log inputs."""
        return self._inputs.model_dump()

    @computed_field
    @property
    def outputs(self) -> Dict[str, Any] | None:
        """Get log outputs."""
        return self._outputs.model_dump()

    @computed_field
    @property
    def feedback(self) -> Dict[str, Any] | None:
        """Get log feedback."""
        return self._feedback.model_dump()

    @computed_field
    @property
    def metadata(self) -> Dict[str, str] | None:
        """Get log metadata."""
        return self._metadata.model_dump()

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
            if self._inputs is not None:
                raise ValueError("Inputs already set")
            self._inputs = IO[Any](inputs)

        if outputs is not None:
            if self._outputs is not None:
                raise ValueError("Outputs already set")
            self._outputs = IO[Any](outputs)

        if feedback is not None:
            if self._feedback is not None:
                raise ValueError("Feedback already set")
            self._feedback = IO[Any](feedback)

        if metadata is not None:
            if self._metadata is not None:
                raise ValueError("Metadata already set")
            self._metadata = IO[Any](metadata)

    @abstractmethod
    async def push(self) -> None:
        """Push log."""

    @abstractmethod
    async def delete(self, id_: str) -> None:
        """Delete log."""

    @asynccontextmanager
    async def observe(self, push: bool = False):
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
            if push:
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

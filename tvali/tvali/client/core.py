"""Core classes for tvali client."""

from typing import Self, Optional, List, Dict, Any
from abc import ABC, abstractmethod
import traceback as tb
import uuid
from datetime import datetime
from pydantic import Field, BaseModel, ConfigDict, computed_field

from ..utils import (
    Record,
    Event,
    Runtime,
    Input,
    Output,
    Feedback,
    Metadata,
    Parameters,
    Tier,
    LogType,
    IOSerializable,
    get_record_type,
)

WORKSPACE_ID = uuid.uuid4()
ENVIRONMENT = "DEV"
VERSION = "0.0.1"


class TvaliBase(BaseModel, ABC):
    """Handler."""

    model_config = ConfigDict(validate_assignment=True)

    event: Event

    runtime_: Runtime = Field(default=None)
    inputs_: List[Input] = Field(default=None, exclude=True)
    outputs_: List[Output] = Field(default=None, exclude=True)
    feedback_: List[Feedback] = Field(default=None, exclude=True)
    metadata_: List[Metadata] = Field(default=None, exclude=True)

    @computed_field
    @property
    def runtime(self) -> Runtime:
        """Runtime."""
        return self.runtime_

    @computed_field
    @property
    def inputs(self) -> List[Input]:
        """Inputs."""
        return self.inputs_

    @computed_field
    @property
    def outputs(self) -> List[Output]:
        """Outputs."""
        return self.outputs_

    @computed_field
    @property
    def feedback(self) -> List[Feedback]:
        """Feedback."""
        return self.feedback_

    @computed_field
    @property
    def metadata(self) -> List[Metadata]:
        """Metadata."""
        return self.metadata_

    @property
    def tier(self) -> Tier:
        """Get tier."""
        return self.event.TIER

    def spawn(
        self,
        name: str,
        parameters: Optional[Parameters] = None,
    ) -> Self:
        """Spawn a new handler as a child of this handler.

        Args:
            name (str): Name of the new handler.
            parameters (Optional[Parameters], optional): Parameters of the new handler.

        Returns:
            Self: The new handler.
        """
        if self.tier.child is None:
            raise ValueError(f"Cannot spawn child of tier {self.tier}")

        return self.__class__(
            event=self.event.spawn(
                name=name,
                parameters=parameters,
                environment=ENVIRONMENT,
                version=VERSION,
            )
        )

    @classmethod
    def trace(cls, name: str, parameters: Optional[Parameters] = None):
        """
        Create a new handler instance with a Tier.SYSTEM event
        and a parent_id that is the WORKSPACE_ID.

        Args:
            name (str): The name of the new handler.
            parameters (Optional[Parameters], optional): Optional parameters for the new handler.

        Returns:
            Self: A new instance of the handler class.
        """
        return cls(
            event=get_record_type(LogType.EVENT, Tier.SYSTEM)(
                parent_id=WORKSPACE_ID,
                name=name,
                parameters=parameters,
                version=VERSION,
                environment=ENVIRONMENT,
            )
        )

    def log(
        self,
        inputs: Optional[Dict[str, Any]] = None,
        outputs: Optional[Dict[str, Any]] = None,
        feedback: Optional[Dict[str, Any]] = None,
        metadata: Optional[Dict[str, str]] = None,
    ):
        """
        Log event details including inputs, outputs, feedback, and metadata.

        This method records various aspects of an event such as inputs, outputs,
        feedback, and metadata. It ensures that each type of data is logged only
        once. If the event has not been set, a ValueError is raised.

        Args:
            inputs (Optional[Dict[str, Any]]): A dictionary of input field names and values to log.
            outputs (Optional[Dict[str, Any]]): A dictionary of output field names and values to log.
            feedback (Optional[Dict[str, Any]]): A dictionary of feedback field names and values to log.
            metadata (Optional[Dict[str, str]]): A dictionary of metadata field names and values to log.

        Raises:
            ValueError: If the event is not set or if any of the inputs, outputs,
                        feedback, or metadata have already been set.
        """
        if self.event is None:
            raise ValueError("Event not set.")

        if inputs:
            if self.inputs_ is not None:
                raise ValueError("Inputs already set")

            self.inputs_ = [
                get_record_type(LogType.INPUT, self.tier)(
                    parent_id=self.event.id,
                    field_name=field_name,
                    field_value=field_value,
                )
                for field_name, field_value in IOSerializable[Dict[str, Any]](
                    inputs
                ).root.items()
            ]

        if outputs:
            if self.outputs_ is not None:
                raise ValueError("Outputs already set")

            self.outputs_ = [
                get_record_type(LogType.OUTPUT, self.tier)(
                    parent_id=self.event.id,
                    field_name=field_name,
                    field_value=field_value,
                )
                for field_name, field_value in IOSerializable[Dict[str, Any]](
                    outputs
                ).root.items()
            ]

        if feedback:
            if self.feedback_ is not None:
                raise ValueError("Feedback already set")

            self.feedback_ = [
                get_record_type(LogType.FEEDBACK, self.tier)(
                    parent_id=self.event.id,
                    field_name=field_name,
                    field_value=field_value,
                )
                for field_name, field_value in IOSerializable[Dict[str, Any]](
                    feedback
                ).root.items()
            ]

        if metadata:
            if self.metadata_ is not None:
                raise ValueError("Metadata already set")

            self.metadata_ = [
                get_record_type(LogType.METADATA, self.tier)(
                    parent_id=self.event.id,
                    field_name=field_name,
                    field_value=field_value,
                )
                for field_name, field_value in IOSerializable[Dict[str, str]](
                    metadata
                ).root.items()
            ]

    def start(self) -> None:
        """
        Initialize the runtime for the event.

        If the runtime has not been set, create a new RUNTIME record and set the start time to the current datetime.
        If the runtime has already started or ended, raise a ValueError.

        Raises:
            ValueError: If the runtime has already started or ended.
        """
        if self.runtime_ is None:
            self.runtime_ = get_record_type(LogType.RUNTIME, self.tier)(
                parent_id=self.event.id
            )
        else:
            if self.runtime_.start_time is not None:
                raise ValueError("Runtime already started")

            if self.runtime_.end_time is not None:
                raise ValueError("Runtime already ended")

        self.runtime_.start_time = datetime.now()

    def end(self) -> None:
        """
        End the runtime for the event.

        If the runtime has not been started, raise a ValueError.
        If the runtime has already ended, raise a ValueError.

        Raises:
            ValueError: If the runtime has not been started or has already ended.
        """
        if self.runtime_ is None:
            raise ValueError("Runtime not started")

        if self.runtime_.end_time is not None:
            raise ValueError("Runtime already ended")

        self.runtime_.end_time = datetime.now()

    async def __aenter__(self):
        await self.push_event()

        self.start()

    async def __aexit__(self, exc_type, exc_value, traceback):
        self.end()

        if exc_type is not None:
            self.runtime_.error_type = exc_type.__name__
            self.runtime_.error_content = tb.format_exc()

        await self.push_io()

    async def push_event(self):
        """Push event."""
        await self.push_records([self.event])

    async def push_io(self):
        """Push IO."""
        await self.push_records(
            [
                self.runtime_,
                *(self.inputs_ or []),
                *(self.outputs_ or []),
                *(self.feedback_ or []),
                *(self.metadata_ or []),
            ]
        )

    @abstractmethod
    async def push_records(self, records: List[Record]) -> None:
        """
        Abstract method to push a list of records asynchronously.

        Args:
            records (List[Record]): A list of records to be pushed.

        This method should be implemented by subclasses to define how the records
        are pushed to their respective destinations (e.g., database, logging service).

        Raises:
            Exception: If an error occurs during the push operation.
        """

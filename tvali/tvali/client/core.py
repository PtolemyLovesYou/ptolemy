"""Core classes for tvali client."""

from typing import Self, Optional, List, Dict, Any
from abc import ABC, abstractmethod
import traceback as tb
import uuid
from datetime import datetime
from pydantic import PrivateAttr, BaseModel

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
    IOSerializable
)

WORKSPACE_ID = uuid.uuid4()
ENVIRONMENT = "DEV"
VERSION = "0.0.1"


class TvaliBase(BaseModel, ABC):
    """Handler."""

    event: Event

    _start_time: datetime = PrivateAttr(default=None)

    _runtime: Runtime = PrivateAttr(default=None)
    _inputs: List[Input] = PrivateAttr(default=None)
    _outputs: List[Output] = PrivateAttr(default=None)
    _feedback: List[Feedback] = PrivateAttr(default=None)
    _metadata: List[Metadata] = PrivateAttr(default=None)

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
            event=Record.build(LogType.EVENT, Tier.SYSTEM)(
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
            if self._inputs is not None:
                raise ValueError("Inputs already set")

            self._inputs = [
                Record.build(LogType.INPUT, self.tier)(
                    parent_id=self.event.id,
                    field_name=field_name,
                    field_value=field_value,
                )
                for field_name, field_value in IOSerializable[Dict[str, Any]](inputs).root.items()
            ]

        if outputs:
            if self._outputs is not None:
                raise ValueError("Outputs already set")

            self._outputs = [
                Record.build(LogType.OUTPUT, self.tier)(
                    parent_id=self.event.id,
                    field_name=field_name,
                    field_value=field_value,
                )
                for field_name, field_value in IOSerializable[Dict[str, Any]](outputs).root.items()
            ]

        if feedback:
            if self._feedback is not None:
                raise ValueError("Feedback already set")

            self._feedback = [
                Record.build(LogType.FEEDBACK, self.tier)(
                    parent_id=self.event.id,
                    field_name=field_name,
                    field_value=field_value,
                )
                for field_name, field_value in IOSerializable[Dict[str, Any]](feedback).root.items()
            ]

        if metadata:
            if self._metadata is not None:
                raise ValueError("Metadata already set")

            self._metadata = [
                Record.build(LogType.METADATA, self.tier)(
                    parent_id=self.event.id,
                    field_name=field_name,
                    field_value=field_value,
                )
                for field_name, field_value in IOSerializable[Dict[str, str]](metadata).root.items()
            ]

    def runtime(
        self,
        start_time: datetime,
        end_time: datetime,
        error_type: Optional[str] = None,
        error_content: Optional[str] = None,
    ):
        """
        Set runtime information for the event.

        Args:
            start_time (datetime): The start time of the event.
            end_time (datetime): The end time of the event.
            error_type (Optional[str]): The type of error that occurred during the event.
            error_content (Optional[str]): The content of the error that occurred during the event.

        Raises:
            ValueError: If the event is not set or if the runtime has already been set.
        """
        if self.event is None:
            raise ValueError("Event not set")

        if self._runtime is not None:
            raise ValueError("Runtime already set")

        self._runtime = Record.build(LogType.RUNTIME, self.tier)(
            parent_id=self.event.id,
            start_time=start_time,
            end_time=end_time,
            error_type=error_type,
            error_content=error_content,
        )

    async def __aenter__(self):
        await self.push_records([self.event])
        if self._start_time is not None:
            raise ValueError("Runtime already started")

        self._start_time = datetime.now()

    async def __aexit__(self, exc_type, exc_value, traceback):
        end_time = datetime.now()

        if exc_type is None:
            error_type = None
            error_content = None
        else:
            error_type = exc_type.__name__
            error_content = tb.format_exc()

        self.runtime(
            self._start_time,
            end_time,
            error_type=error_type,
            error_content=error_content,
        )

        await self.push_records(
            [
                self._runtime,
                *(self._inputs or []),
                *(self._outputs or []),
                *(self._feedback or []),
                *(self._metadata or []),
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

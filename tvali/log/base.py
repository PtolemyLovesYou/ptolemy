"""Log base class."""
from typing import ClassVar, Optional, Type
from abc import ABC, abstractmethod
import traceback
from contextlib import contextmanager
from uuid import uuid4
from datetime import datetime
from pydantic import ConfigDict, BaseModel, PrivateAttr, Field, computed_field
from .types import IO, ID, Metadata, Time, Tier
from ..config import Config


class _Log(BaseModel, ABC):
    """Log Base class."""
    TIER: ClassVar[Tier]

    model_config = ConfigDict(validate_assignment=True)

    _start_time: Time = PrivateAttr(default=None)
    _end_time: Time = PrivateAttr(default=None)
    _recorded: bool = PrivateAttr(default=False)

    id: ID = Field(default_factory=uuid4)
    name: str
    parameters: Optional[IO] = None
    error_type: Optional[str] = None
    error_content: Optional[str] = None

    inputs: Optional[IO] = None
    outputs: Optional[IO] = None
    feedback: Optional[IO] = None
    metadata: Optional[IO] = None

    @computed_field
    @property
    def version(self) -> Optional[str]:
        return Config.version

    @computed_field
    @property
    def environment(self) -> Optional[str]:
        return Config.environment

    @computed_field
    @property
    def start_time(self) -> Optional[Time]:
        """
        Start time.
        """
        return self._start_time

    @computed_field
    @property
    def end_time(self) -> Optional[Time]:
        """
        End time.
        """
        return self._end_time

    def event(self) -> dict:
        return self.model_dump(
            exclude=["inputs", "outputs", "feedback", "metadata"],
            exclude_none=True
            )

    def start(self) -> None:
        if self._start_time is not None:
            raise ValueError("Start time already set")

        self._start_time = datetime.now()

    def stop(self) -> None:
        if self._end_time is not None:
            raise ValueError("End time already set")

        self._end_time = datetime.now()

    @contextmanager
    def execute(self):
        """
        Execute a block of code and log its execution.

        This context manager sets the start and end times of the log, and
        captures any exceptions that occur during execution. If an exception
        occurs, the error type and content are recorded, and the exception is
        re-raised.

        This is a context manager, so it should be used with a `with` statement:

        >>> with log.execute():
        ...     # do something

        If an exception occurs, it will be re-raised after the log is updated.
        """
        if not self._start_time:
            self.start()
        try:
            yield
        except Exception as e:
            self.error_content = traceback.format_exc()
            self.error_type = type(e).__name__

            raise e
        finally:
            if not self._end_time:
                self.stop()

    def log(
        self,
        inputs: Optional[IO] = None,
        outputs: Optional[IO] = None,
        feedback: Optional[IO] = None,
        metadata: Optional[Metadata] = None
        ):
        if inputs:
            if self.inputs is not None:
                raise ValueError("Inputs already set")
            self.inputs = inputs

        if outputs:
            if self.outputs is not None:
                raise ValueError("Outputs already set")
            self.outputs = outputs

        if feedback:
            if self.feedback is not None:
                raise ValueError("Feedback already set")
            self.feedback = feedback

        if metadata:
            if self.metadata is not None:
                raise ValueError("Metadata already set")
            self.metadata = metadata

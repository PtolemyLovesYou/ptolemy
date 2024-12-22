"""Client."""

from typing import List, Self, Optional, Any, Annotated
import time
import traceback as tb
from pydantic import BaseModel, PrivateAttr, Field, ConfigDict
from ..engine.engine import Engine
from ..engine.grpc import PtolemyEngine
from ..utils import (
    LogType,
    Tier,
    ID,
)
from .._core import ( # pylint: disable=no-name-in-module
    ProtoRecord
)


class Ptolemy(BaseModel):
    """Ptolemy client."""
    model_config = ConfigDict(arbitrary_types_allowed=True)

    _tier: Tier = PrivateAttr(default=None)

    _event: ProtoRecord = PrivateAttr(default=None)
    _runtime: ProtoRecord = PrivateAttr(default=None)
    _inputs: List[ProtoRecord] = PrivateAttr(default=None)
    _outputs: List[ProtoRecord] = PrivateAttr(default=None)
    _feedback: List[ProtoRecord] = PrivateAttr(default=None)
    _metadata: List[ProtoRecord] = PrivateAttr(default=None)

    _start_time: float = PrivateAttr(default=None)
    _end_time: float = PrivateAttr(default=None)

    engine: Annotated[Engine, Field(default_factory=PtolemyEngine)]
    workspace_id: ID

    autoflush: bool = False

    def start(self) -> Self:
        """
        Start a runtime.

        Raises:
            ValueError: If already started.
        """
        if self._start_time is not None:
            raise ValueError("Already started")
        self._start_time = time.time()
        return self

    def stop(self) -> Self:
        """
        stop a runtime.

        Raises:
            ValueError: If already ended.

        Returns:
            Self: The Ptolemy instance.
        """
        if self._end_time is not None:
            raise ValueError("Already ended")

        self._end_time = time.time()
        return self

    def __enter__(self):
        if self._start_time is not None:
            raise ValueError("Already started")

        self.engine.queue_event(self._event)

        self.start()

    def __exit__(self, exc_type, exc_value, traceback):
        # End runtime if not already ended
        if self._end_time is None:
            self.stop()

        error_type = None
        error_content = None

        if exc_type is not None:
            error_type = exc_type.__name__
            error_content = tb.format_exc()

        self.runtime(
            start_time=self._start_time,
            end_time=self._end_time,
            error_type=error_type,
            error_content=error_content,
        )

        self.engine.queue(  # pylint: disable=no-member
            [
                self._runtime,
                *(self._inputs or []),
                *(self._outputs or []),
                *(self._feedback or []),
                *(self._metadata or []),
            ]
        )

        if self._tier == Tier.SYSTEM and self.autoflush:
            self.engine.flush()  # pylint: disable=no-member

    def tier(self, tier: Tier) -> Self:
        """Set tier."""
        if self._tier is not None:
            raise ValueError("Tier already set")
        self._tier = tier
        return self

    def child(
        self,
        name: str,
        parameters: Optional[dict] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> "Ptolemy":
        """Spawn a child log."""
        if self._tier is None:
            raise ValueError("Tier not set")

        if self._tier.child is None:
            raise ValueError(f"Cannot spawn child of tier {self._tier}")

        return (
            Ptolemy(engine=self.engine, workspace_id=self.workspace_id, autoflush=self.autoflush)
            .tier(self._tier.child)
            .event(
                name=name,
                parameters=parameters,
                version=version,
                environment=environment,
                parent_id=self._event.id,
            )
        )

    def trace(
        self,
        name: str,
        parameters: Optional[dict] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> "Ptolemy":
        """Start a new trace."""
        return (
            Ptolemy(engine=self.engine, workspace_id=self.workspace_id, autoflush=self.autoflush)
            .tier(Tier.SYSTEM)
            .event(
                parent_id=self.workspace_id,
                name=name,
                parameters=parameters,
                version=version,
                environment=environment,
            )
        )

    def event(
        self,
        name: str,
        parameters: Optional[dict] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
        parent_id: ID = None,
    ) -> Self:
        """
        Set the event.

        Args:
            name: The name of the event.
            parameters: The parameters of the event.
            version: The version of the event.
            environment: The environment of the event.

        Returns:
            The current instance.

        Raises:
            ValueError: If the event is already set.
        """
        if self._event is not None:
            raise ValueError("Event already set")

        if self._tier != Tier.SYSTEM and parent_id is None:
            raise ValueError("Parent ID is required for non-system events")

        self._event = ProtoRecord.event(
            self._tier.value,
            name,
            parent_id.hex or self.workspace_id,
            id=None,
            parameters=parameters,
            version=version,
            environment=environment,
            )

        return self

    def runtime(
        self,
        start_time: float,
        end_time: float,
        error_type: Optional[str] = None,
        error_content: Optional[str] = None,
    ) -> Self:
        """
        Set the runtime for the event.

        This method records the start and end times of the event's execution, along with
        any errors encountered during that period. It ensures that the runtime is only
        set once for an event.

        Args:
            start_time: The timestamp when the event started.
            end_time: The timestamp when the event ended.
            error_type: Optional; the type of error encountered, if any.
            error_content: Optional; detailed content or message of the error, if any.

        Returns:
            The Ptolemy client object.

        Raises:
            ValueError: If the runtime has already been set.
        """
        if self._event is None:
            raise ValueError("Event not set")

        if self._runtime is not None:
            raise ValueError("Runtime already set")

        self._runtime = ProtoRecord.runtime(
            self._tier.value,
            self._event.id,
            start_time=start_time,
            end_time=end_time,
            id=None,
            error_type=error_type,
            error_content=error_content,
        )

        return self

    def inputs(self, **kwargs: Any) -> Self:
        """
        Set the inputs for the event.

        The inputs are a dictionary of key-value pairs where the keys are strings
        and the values are arbitrary objects. The inputs are used to store any
        data that was used to generate the event.

        :param **kwargs: The keyword arguments to set as inputs.
        :raises ValueError: If the inputs have already been set.
        :return: The Ptolemy client object.
        """
        if self._event is None:
            raise ValueError("Event not set")

        if self._inputs is not None:
            raise ValueError("Inputs already set")

        self._inputs = [
            ProtoRecord.io(
                self._tier,
                LogType.INPUT.value,
                self._event.id,
                k,
                v
            ) for k, v in kwargs.items()
        ]

        return self

    def outputs(self, **kwargs: Any) -> Self:
        """
        Set the outputs for the event.

        The outputs are a dictionary of key-value pairs where the keys are strings
        and the values are arbitrary objects. The outputs are used to store any
        data that is produced by the event.

        The outputs are only set once, and attempting to call this method more than
        once will raise a ValueError.

        Args:
            **kwargs: The outputs to be set as key-value pairs.

        Returns:
            The current instance.

        Raises:
            ValueError: If the outputs are already set.
        """
        if self._event is None:
            raise ValueError("Event not set")

        if self._outputs is not None:
            raise ValueError("Outputs already set")

        self._outputs = [
            ProtoRecord.io(
                self._tier,
                self._event.id,
                LogType.OUTPUT.value,
                k,
                v
            ) for k, v in kwargs.items()
        ]

        return self

    def feedback(self, **kwargs: Any) -> Self:
        """
        Set the feedback for the event.

        The feedback is a dictionary of key-value pairs where the keys are strings
        and the values are arbitrary objects. The feedback is used to store any
        additional information that is not captured by the inputs, outputs, or runtime.

        The feedback is only set once, and attempting to call this method more than
        once will raise a ValueError.
        """
        if self._event is None:
            raise ValueError("Event not set")

        if self._feedback is not None:
            raise ValueError("Feedback already set")

        self._feedback = [
            ProtoRecord.io(
                self._tier,
                self._event.id,
                LogType.FEEDBACK.value,
                k,
                v
            ) for k, v in kwargs.items()
        ]

        return self

    def metadata(self, **kwargs: str) -> Self:
        """
        Set the metadata for the event.

        The metadata is a dictionary of key-value pairs that describes the event.
        The keys are strings and the values are strings.

        The metadata is optional and can be set multiple times.

        Args:
            **kwargs: The metadata to be set.

        Returns:
            The current instance.

        Raises:
            ValueError: If the metadata is already set.
        """
        if self._event is None:
            raise ValueError("Event not set")

        if self._metadata is not None:
            raise ValueError("Metadata already set")

        self._metadata = [
            ProtoRecord.metadata(
                self._tier,
                self._event.id,
                LogType.METADATA.value,
                k,
                v
            ) for k, v in kwargs.items()
        ]

        return self

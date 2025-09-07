from __future__ import annotations

from uuid import uuid4, UUID
import time
from typing import Optional, Any, Type, List, TYPE_CHECKING
from types import TracebackType
import traceback
from pydantic import BaseModel, Field

from .tier import Tier
from .io import Runtime, IO, Parameters

if TYPE_CHECKING:
    from .client.base import PtolemyBase

def _format_err(
    exc_type: Optional[Type[BaseException]],
    exc_value: Optional[Exception],
    tb: Optional[TracebackType],
) -> tuple[Optional[str], Optional[str]]:
    if exc_type is not None:
        format_result = "".join(traceback.format_exception(exc_type, exc_value, tb))
        return exc_type.__name__, format_result

    return None, None

class Trace(BaseModel):
    """Trace."""

    client: "PtolemyBase" = Field(exclude=True, repr=False)

    subject_id: UUID
    parent_id: UUID
    id_: UUID = Field(default_factory=uuid4, alias="id")

    tier: Tier
    name: str

    parameters: Optional[Parameters] = Field(default=None)

    version: Optional[str] = Field(default=None)
    environment: Optional[str] = Field(default=None)

    start_time: Optional[float] = None

    runtime_: Optional[Runtime] = Field(default=None)

    inputs_: Optional[List[IO[Any]]] = Field(default=None)
    outputs_: Optional[List[IO[Any]]] = Field(default=None)
    feedback_: Optional[List[IO[Any]]] = Field(default=None)
    metadata_: Optional[List[IO[str]]] = Field(default=None)

    def start(self):
        """Start event trace."""

        if self.start_time is not None:
            raise ValueError("Runtime already started.")

        self.start_time = time.time()

    def end(
        self,
        exc_type: Optional[BaseException],
        exc_value: Optional[Exception],
        tb: Optional[TracebackType],
    ):
        """End runtime log."""

        if self.start_time is None:
            raise ValueError("Runtime not started yet.")

        if self.runtime_ is not None:
            raise ValueError("Runtime already ended.")

        end_time = time.time()
        error_type, error_content = _format_err(exc_type, exc_value, tb)

        self.runtime_ = Runtime(
            subject_id=self.subject_id,
            event_id=self.id_,
            start_time=self.start_time,
            end_time=end_time,
            error_type=error_type,
            error_content=error_content,
        )

    async def __aenter__(self):
        self.start()

    async def __aexit__(self, exc_type, exc_value, tb):
        self.end(exc_type, exc_value, tb)

        await self.client.add_trace(self)

    def __enter__(self):
        self.start()

    def __exit__(self, exc_type, exc_value, tb):
        self.end(exc_type, exc_value, tb)

        self.client.add_trace_blocking(self)

    def child(
        self,
        name: str,
        parameters: Optional[Parameters] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> "Trace":
        """Create child trace."""

        if self.tier == Tier.SUBCOMPONENT:
            raise ValueError("Cannot create a child trace of a subcomponent.")

        return Trace(
            client=self.client,
            subject_id=self.subject_id,
            parent_id=self.id_,
            tier=self.tier.child(),
            name=name,
            parameters=parameters,
            version=version or self.version,
            environment=environment or self.environment,
        )

    def _set_singleton_field(
        self, attr: str, attr_name: str, cls: type[BaseModel], **kwargs
    ):
        if getattr(self, attr) is not None:
            raise ValueError(f"{attr_name} already set.")

        setattr(
            self,
            attr,
            [
                cls(
                    subject_id=self.subject_id,
                    event_id=self.id_,
                    field_name=k,
                    field_value=v,
                )
                for k, v in kwargs.items()
                if v is not None
            ],
        )

    def inputs(self, **kwargs: Any):
        """Set inputs."""

        self._set_singleton_field("inputs_", "Inputs", IO[Any], **kwargs)

    def outputs(self, **kwargs: Any):
        """Set outputs."""

        self._set_singleton_field("outputs_", "Outputs", IO[Any], **kwargs)

    def feedback(self, **kwargs: Any):
        """Set feedback."""

        self._set_singleton_field("feedback_", "Feedback", IO[Any], **kwargs)

    def metadata(self, **kwargs: str):
        """Set metadata."""

        self._set_singleton_field("metadata_", "Metadata", IO[str], **kwargs)

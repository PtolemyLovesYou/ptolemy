"""Ptolemy Client."""

from typing import Dict, Any, Optional, List
import time
import logging
import traceback
from uuid import UUID, uuid4
from pydantic import BaseModel, Field, InstanceOf

from .tier import Tier
from .io import IO, Runtime
from .._core import RecordExporter

logger = logging.getLogger(__name__)

Parameters = Dict[str, Any]

class Ptolemy(BaseModel):
    """Ptolemy Client."""

    base_url: str

    workspace_id: UUID
    workspace_name: str

    client: InstanceOf[RecordExporter]

    def add_trace(self, trace: "Trace"):
        """Send trace."""
        # TODO: Batching, retries, etc.

        try:
            self.client.send_trace(trace)
        # Thrown when invalid trace is sent
        except AttributeError as e:
            logger.error("Invalid trace type: %s", trace.__class__.__name__)
        except ConnectionError as e:
            logger.error("Error sending trace %s: %s", trace.id_, e)

    def trace(
        self,
        name: str,
        parameters: Optional[Parameters],
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> "Trace":
        """Create new trace."""

        return Trace(
            client=self,
            tier=Tier.SYSTEM,
            parent_id=self.workspace_id,
            name=name,
            parameters=parameters,
            version=version,
            environment=environment,
        )

def connect(base_url: str) -> Ptolemy:
    """
    Connect to Ptolemy client.
    """
    client = RecordExporter(base_url)
    workspace_id, workspace_name = client.get_workspace_info()

    return Ptolemy(
        base_url=base_url,
        client=RecordExporter(base_url),
        workspace_id=workspace_id,
        workspace_name=workspace_name,
    )

def _format_err(exc_type, exc_value, tb) -> tuple[Optional[str], Optional[str]]:
    if exc_type is not None:
        format_result = "".join(traceback.format_exception(exc_type, exc_value, tb))
        return exc_type.__name__, format_result

    return None, None

class Trace(BaseModel):
    """Trace."""

    client: Ptolemy = Field(exclude=True, repr=False)

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

    def end(self, exc_type, exc_value, tb):
        """End runtime log."""

        if self.start_time is None:
            raise ValueError("Runtime not started yet.")

        if self.runtime_ is not None:
            raise ValueError("Runtime already ended.")

        end_time = time.time()
        error_type, error_content = _format_err(exc_type, exc_value, tb)

        self.runtime_ = Runtime(
            parent_id=self.id_,
            start_time=self.start_time,
            end_time=end_time,
            error_type=error_type,
            error_content=error_content,
        )

    def __enter__(self):
        self.start()

    def __exit__(self, exc_type, exc_value, tb):
        self.end(exc_type, exc_value, tb)

        self.client.add_trace(self)

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
                cls(parent_id=self.id_, field_name=k, field_value=v)
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

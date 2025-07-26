"""Ptolemy Client."""

from typing import Dict, Any, Optional, List, Self
from uuid import UUID, uuid4
from pydantic import BaseModel, Field, PrivateAttr, model_validator

from .tier import Tier
from .io import IO, Runtime
from .._core import RecordExporter

Parameters = Dict[str, Any]

class Ptolemy(BaseModel):
    """Ptolemy Client."""

    base_url: str
    api_key: str

    _workspace_id: Optional[UUID] = PrivateAttr(None)
    _client: Optional[RecordExporter] = PrivateAttr(None)

    @property
    def workspace_id(self) -> UUID:
        """Get workspace id."""
        if self._workspace_id is None:
            raise ValueError("Workspace ID must be set.")

        return self._workspace_id

    @model_validator(mode="after")
    def connect_to_client(self) -> Self:
        """Connect to client."""
        self._client = RecordExporter(self.base_url)

        return self

    def add_trace(self, trace: "Trace"):
        """Send trace."""

        self._client.send_trace(trace)

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

    runtime_: Optional[Runtime] = Field(default=None)

    inputs_: Optional[List[IO[Any]]] = Field(default=None)
    outputs_: Optional[List[IO[Any]]] = Field(default=None)
    feedback_: Optional[List[IO[Any]]] = Field(default=None)
    metadata_: Optional[List[IO[str]]] = Field(default=None)

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

    @classmethod
    def new(
        cls,
        client: Ptolemy,
        name: str,
        parameters: Optional[Parameters] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> "Trace":
        """Create new trace."""

        return cls(
            client=client,
            tier=Tier.SYSTEM,
            parent_id=client.workspace_id,
            name=name,
            parameters=parameters,
            version=version,
            environment=environment,
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

    def runtime(
        self,
        start_time: float,
        end_time: float,
        error_type: Optional[str] = None,
        error_content: Optional[str] = None,
    ):
        if self.runtime_ is not None:
            raise ValueError("Runtime already exists.")

        self.runtime_ = Runtime(
            parent_id=self.id_,
            start_time=start_time,
            end_time=end_time,
            error_type=error_type,
            error_content=error_content,
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

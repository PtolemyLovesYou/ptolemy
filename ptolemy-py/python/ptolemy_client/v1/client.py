"""Ptolemy Client."""

from typing import Dict, Any, Optional, Self, List, TypeVar, Generic
from enum import StrEnum
from uuid import UUID, uuid4
from pydantic import BaseModel, Field, PrivateAttr

Parameters = Dict[str, Any]

T = TypeVar("T")

class Tier(StrEnum):
    """Tier."""

    SYSTEM = "system"
    SUBSYSTEM = "subsystem"
    COMPONENT = "component"
    SUBCOMPONENT = "subcomponent"

    def child(self) -> Self:
        """Get child tier."""

        if self == Tier.SYSTEM:
            return Tier.SUBSYSTEM

        if self == Tier.SUBSYSTEM:
            return Tier.COMPONENT

        if self == Tier.COMPONENT:
            return Tier.SUBCOMPONENT

        raise ValueError("Cannot create a child trace of a subcomponent.")

class Ptolemy(BaseModel):
    """Ptolemy Client."""

    base_url: str
    api_key: str

    _workspace_id: Optional[UUID] = PrivateAttr(None)

    @property
    def workspace_id(self) -> UUID:
        """Get workspace id."""
        if self._workspace_id is None:
            raise ValueError("Workspace ID must be set.")

        return self._workspace_id

class IO(BaseModel, Generic[T]):
    """IO object."""

    parent_id: UUID
    id_: UUID = Field(default_factory=uuid4, alias="id")
    field_name: str
    field_value: T

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
    end_time: Optional[float] = None

    error_type: Optional[str] = None
    error_value: Optional[str] = None

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

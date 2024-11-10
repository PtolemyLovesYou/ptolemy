"""Component event schemas."""

from .core import (
    EventLogMixin,
    InputLogMixin,
    RuntimeLogMixin,
    OutputLogMixin,
    FeedbackLogMixin,
    MetadataLogMixin,
    RecordSchemaMixin,
    CreateSchemaMixin,
    SubsystemDependentMixin,
    ComponentDependentMixin,
)


class ComponentEventBase(EventLogMixin, SubsystemDependentMixin):
    """Component event base schema."""


class ComponentEvent(ComponentEventBase, RecordSchemaMixin):
    """Component event schema."""


class ComponentEventCreate(ComponentEventBase, CreateSchemaMixin):
    """Component event create schema."""


class ComponentRuntimeBase(RuntimeLogMixin, ComponentDependentMixin):
    """Component event runtime base schema."""


class ComponentRuntime(ComponentRuntimeBase, RecordSchemaMixin):
    """Component event runtime schema."""


class ComponentRuntimeCreate(ComponentRuntimeBase, CreateSchemaMixin):
    """Component event runtime create schema."""


class ComponentInputBase(InputLogMixin, ComponentDependentMixin):
    """Component event input base schema."""


class ComponentInput(ComponentInputBase, RecordSchemaMixin):
    """Component event input schema."""


class ComponentInputCreate(ComponentInputBase, CreateSchemaMixin):
    """Component event input create schema."""


class ComponentOutputBase(OutputLogMixin, ComponentDependentMixin):
    """Component event output base schema."""


class ComponentOutput(ComponentOutputBase, RecordSchemaMixin):
    """Component event output schema."""


class ComponentOutputCreate(ComponentOutputBase, CreateSchemaMixin):
    """Component event output create schema."""


class ComponentFeedbackBase(FeedbackLogMixin, ComponentDependentMixin):
    """Component event feedback base schema."""


class ComponentFeedback(ComponentFeedbackBase, RecordSchemaMixin):
    """Component event feedback schema."""


class ComponentFeedbackCreate(ComponentFeedbackBase, CreateSchemaMixin):
    """Component event feedback create schema."""


class ComponentMetadataBase(MetadataLogMixin, ComponentDependentMixin):
    """Component event metadata base schema."""


class ComponentMetadata(ComponentMetadataBase, RecordSchemaMixin):
    """Component event metadata schema."""


class ComponentMetadataCreate(ComponentMetadataBase, CreateSchemaMixin):
    """Component event metadata create schema."""

"""Subcomponent event schemas."""

from .core import (
    EventLogMixin,
    InputLogMixin,
    RuntimeLogMixin,
    OutputLogMixin,
    FeedbackLogMixin,
    MetadataLogMixin,
    RecordSchemaMixin,
    CreateSchemaMixin,
    ComponentDependentMixin,
    SubcomponentDependentMixin,
)


class SubcomponentEventBase(EventLogMixin, ComponentDependentMixin):
    """Subcomponent event base schema."""


class SubcomponentEvent(SubcomponentEventBase, RecordSchemaMixin):
    """Subcomponent event schema."""


class SubcomponentEventCreate(SubcomponentEventBase, CreateSchemaMixin):
    """Subcomponent event create schema."""


class SubcomponentRuntimeBase(RuntimeLogMixin, SubcomponentDependentMixin):
    """Subcomponent event runtime base schema."""


class SubcomponentRuntime(SubcomponentRuntimeBase, RecordSchemaMixin):
    """Subcomponent event runtime schema."""


class SubcomponentRuntimeCreate(SubcomponentRuntimeBase, CreateSchemaMixin):
    """Subcomponent event runtime create schema."""


class SubcomponentInputBase(InputLogMixin, SubcomponentDependentMixin):
    """Subcomponent event input base schema."""


class SubcomponentInput(SubcomponentInputBase, RecordSchemaMixin):
    """Subcomponent event input schema."""


class SubcomponentInputCreate(SubcomponentInputBase, CreateSchemaMixin):
    """Subcomponent event input create schema."""


class SubcomponentOutputBase(OutputLogMixin, SubcomponentDependentMixin):
    """Subcomponent event output base schema."""


class SubcomponentOutput(SubcomponentOutputBase, RecordSchemaMixin):
    """Subcomponent event output schema."""


class SubcomponentOutputCreate(SubcomponentOutputBase, CreateSchemaMixin):
    """Subcomponent event output create schema."""


class SubcomponentFeedbackBase(FeedbackLogMixin, SubcomponentDependentMixin):
    """Subcomponent event feedback base schema."""


class SubcomponentFeedback(SubcomponentFeedbackBase, RecordSchemaMixin):
    """Subcomponent event feedback schema."""


class SubcomponentFeedbackCreate(SubcomponentFeedbackBase, CreateSchemaMixin):
    """Subcomponent event feedback create schema."""


class SubcomponentMetadataBase(MetadataLogMixin, SubcomponentDependentMixin):
    """Subcomponent event metadata base schema."""


class SubcomponentMetadata(SubcomponentMetadataBase, RecordSchemaMixin):
    """Subcomponent event metadata schema."""


class SubcomponentMetadataCreate(SubcomponentMetadataBase, CreateSchemaMixin):
    """Subcomponent event metadata create schema."""

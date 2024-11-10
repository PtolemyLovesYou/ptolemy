"""System event schemas."""

from .core import (
    EventLogMixin,
    InputLogMixin,
    RuntimeLogMixin,
    OutputLogMixin,
    FeedbackLogMixin,
    MetadataLogMixin,
    RecordSchemaMixin,
    CreateSchemaMixin,
    SystemDependentMixin,
)


class SystemEventBase(EventLogMixin):
    """System event base schema."""


class SystemEvent(SystemEventBase, RecordSchemaMixin):
    """System event schema."""


class SystemEventCreate(SystemEventBase, CreateSchemaMixin):
    """System event create schema."""


class SystemRuntimeBase(RuntimeLogMixin, SystemDependentMixin):
    """System event runtime base schema."""


class SystemRuntime(SystemRuntimeBase, RecordSchemaMixin):
    """System event runtime schema."""


class SystemRuntimeCreate(SystemRuntimeBase, CreateSchemaMixin):
    """System event runtime create schema."""


class SystemInputBase(InputLogMixin, SystemDependentMixin):
    """System event input base schema."""


class SystemInput(SystemInputBase, RecordSchemaMixin):
    """System event input schema."""


class SystemInputCreate(SystemInputBase, CreateSchemaMixin):
    """System event input create schema."""


class SystemOutputBase(OutputLogMixin, SystemDependentMixin):
    """System event output base schema."""


class SystemOutput(SystemOutputBase, RecordSchemaMixin):
    """System event output schema."""


class SystemOutputCreate(SystemOutputBase, CreateSchemaMixin):
    """System event output create schema."""


class SystemFeedbackBase(FeedbackLogMixin, SystemDependentMixin):
    """System event feedback base schema."""


class SystemFeedback(SystemFeedbackBase, RecordSchemaMixin):
    """System event feedback schema."""


class SystemFeedbackCreate(SystemFeedbackBase, CreateSchemaMixin):
    """System event feedback create schema."""


class SystemMetadataBase(MetadataLogMixin, SystemDependentMixin):
    """System event metadata base schema."""


class SystemMetadata(SystemMetadataBase, RecordSchemaMixin):
    """System event metadata schema."""


class SystemMetadataCreate(SystemMetadataBase, CreateSchemaMixin):
    """System event metadata create schema."""

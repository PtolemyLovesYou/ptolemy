"""Subsystem event schemas."""

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
    SubsystemDependentMixin,
)


class SubsystemEventBase(EventLogMixin, SystemDependentMixin):
    """Subsystem event base schema."""


class SubsystemEvent(SubsystemEventBase, RecordSchemaMixin):
    """Subsystem event schema."""


class SubsystemEventCreate(SubsystemEventBase, CreateSchemaMixin):
    """Subsystem event create schema."""


class SubsystemRuntimeBase(RuntimeLogMixin, SubsystemDependentMixin):
    """Subsystem event runtime base schema."""


class SubsystemRuntime(SubsystemRuntimeBase, RecordSchemaMixin):
    """Subsystem event runtime schema."""


class SubsystemRuntimeCreate(SubsystemRuntimeBase, CreateSchemaMixin):
    """Subsystem event runtime create schema."""


class SubsystemInputBase(InputLogMixin, SubsystemDependentMixin):
    """Subsystem event input base schema."""


class SubsystemInput(SubsystemInputBase, RecordSchemaMixin):
    """Subsystem event input schema."""


class SubsystemInputCreate(SubsystemInputBase, CreateSchemaMixin):
    """Subsystem event input create schema."""


class SubsystemOutputBase(OutputLogMixin, SubsystemDependentMixin):
    """Subsystem event output base schema."""


class SubsystemOutput(SubsystemOutputBase, RecordSchemaMixin):
    """Subsystem event output schema."""


class SubsystemOutputCreate(SubsystemOutputBase, CreateSchemaMixin):
    """Subsystem event output create schema."""


class SubsystemFeedbackBase(FeedbackLogMixin, SubsystemDependentMixin):
    """Subsystem event feedback base schema."""


class SubsystemFeedback(SubsystemFeedbackBase, RecordSchemaMixin):
    """Subsystem event feedback schema."""


class SubsystemFeedbackCreate(SubsystemFeedbackBase, CreateSchemaMixin):
    """Subsystem event feedback create schema."""


class SubsystemMetadataBase(MetadataLogMixin, SubsystemDependentMixin):
    """Subsystem event metadata base schema."""


class SubsystemMetadata(SubsystemMetadataBase, RecordSchemaMixin):
    """Subsystem event metadata schema."""


class SubsystemMetadataCreate(SubsystemMetadataBase, CreateSchemaMixin):
    """Subsystem event metadata create schema."""

"""Subcomponent event schemas."""

from .core import (
    EventBase,
    EventInputBase,
    EventRuntimeBase,
    EventOutputBase,
    EventFeedbackBase,
    EventMetadataBase,
    Record,
    Create,
    ComponentDependent,
    SubcomponentDependent,
)


class SubcomponentEventBase(EventBase, ComponentDependent):
    """Subcomponent event base schema."""


class SubcomponentEvent(SubcomponentEventBase, Record):
    """Subcomponent event schema."""


class SubcomponentEventCreate(SubcomponentEventBase, Create):
    """Subcomponent event create schema."""


class SubcomponentRuntimeBase(EventRuntimeBase, SubcomponentDependent):
    """Subcomponent event runtime base schema."""


class SubcomponentRuntime(SubcomponentRuntimeBase, Record):
    """Subcomponent event runtime schema."""


class SubcomponentRuntimeCreate(SubcomponentRuntimeBase, Create):
    """Subcomponent event runtime create schema."""


class SubcomponentInputBase(EventInputBase, SubcomponentDependent):
    """Subcomponent event input base schema."""


class SubcomponentInput(SubcomponentInputBase, Record):
    """Subcomponent event input schema."""


class SubcomponentInputCreate(SubcomponentInputBase, Create):
    """Subcomponent event input create schema."""


class SubcomponentOutputBase(EventOutputBase, SubcomponentDependent):
    """Subcomponent event output base schema."""


class SubcomponentOutput(SubcomponentOutputBase, Record):
    """Subcomponent event output schema."""


class SubcomponentOutputCreate(SubcomponentOutputBase, Create):
    """Subcomponent event output create schema."""


class SubcomponentFeedbackBase(EventFeedbackBase, SubcomponentDependent):
    """Subcomponent event feedback base schema."""


class SubcomponentFeedback(SubcomponentFeedbackBase, Record):
    """Subcomponent event feedback schema."""


class SubcomponentFeedbackCreate(SubcomponentFeedbackBase, Create):
    """Subcomponent event feedback create schema."""


class SubcomponentMetadataBase(EventMetadataBase, SubcomponentDependent):
    """Subcomponent event metadata base schema."""


class SubcomponentMetadata(SubcomponentMetadataBase, Record):
    """Subcomponent event metadata schema."""


class SubcomponentMetadataCreate(SubcomponentMetadataBase, Create):
    """Subcomponent event metadata create schema."""

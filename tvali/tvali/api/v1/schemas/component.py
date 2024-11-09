"""Component event schemas."""

from .core import (
    EventBase,
    EventInputBase,
    EventRuntimeBase,
    EventOutputBase,
    EventFeedbackBase,
    EventMetadataBase,
    Record,
    Create,
    SubsystemDependent,
    ComponentDependent,
)


class ComponentEventBase(EventBase, SubsystemDependent):
    """Component event base schema."""


class ComponentEvent(ComponentEventBase, Record):
    """Component event schema."""


class ComponentEventCreate(ComponentEventBase, Create):
    """Component event create schema."""


class ComponentRuntimeBase(EventRuntimeBase, ComponentDependent):
    """Component event runtime base schema."""


class ComponentRuntime(ComponentRuntimeBase, Record):
    """Component event runtime schema."""


class ComponentRuntimeCreate(ComponentRuntimeBase, Create):
    """Component event runtime create schema."""


class ComponentInputBase(EventInputBase, ComponentDependent):
    """Component event input base schema."""


class ComponentInput(ComponentInputBase, Record):
    """Component event input schema."""


class ComponentInputCreate(ComponentInputBase, Create):
    """Component event input create schema."""


class ComponentOutputBase(EventOutputBase, ComponentDependent):
    """Component event output base schema."""


class ComponentOutput(ComponentOutputBase, Record):
    """Component event output schema."""


class ComponentOutputCreate(ComponentOutputBase, Create):
    """Component event output create schema."""


class ComponentFeedbackBase(EventFeedbackBase, ComponentDependent):
    """Component event feedback base schema."""


class ComponentFeedback(ComponentFeedbackBase, Record):
    """Component event feedback schema."""


class ComponentFeedbackCreate(ComponentFeedbackBase, Create):
    """Component event feedback create schema."""


class ComponentMetadataBase(EventMetadataBase, ComponentDependent):
    """Component event metadata base schema."""


class ComponentMetadata(ComponentMetadataBase, Record):
    """Component event metadata schema."""


class ComponentMetadataCreate(ComponentMetadataBase, Create):
    """Component event metadata create schema."""

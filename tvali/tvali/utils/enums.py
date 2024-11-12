"""Enums."""

from enum import StrEnum


class LogType(StrEnum):
    """Log types."""

    EVENT = "event"
    RUNTIME = "runtime"
    INPUT = "input"
    OUTPUT = "output"
    FEEDBACK = "feedback"
    METADATA = "metadata"


class Tier(StrEnum):
    """Tier types."""

    SYSTEM = "system"
    SUBSYSTEM = "subsystem"
    COMPONENT = "component"
    SUBCOMPONENT = "subcomponent"

"""Enums."""

from typing import Optional
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
    """Tiers."""

    SYSTEM = "system"
    SUBSYSTEM = "subsystem"
    COMPONENT = "component"
    SUBCOMPONENT = "subcomponent"

    @property
    def parent(self) -> Optional["Tier"]:
        """Get parent tier."""
        return {
            Tier.SUBCOMPONENT: Tier.COMPONENT,
            Tier.COMPONENT: Tier.SUBSYSTEM,
            Tier.SUBSYSTEM: Tier.SYSTEM,
        }.get(self)

    @property
    def child(self) -> Optional["Tier"]:
        """Get child tier."""
        return {
            Tier.SYSTEM: Tier.SUBSYSTEM,
            Tier.SUBSYSTEM: Tier.COMPONENT,
            Tier.COMPONENT: Tier.SUBCOMPONENT,
        }.get(self)

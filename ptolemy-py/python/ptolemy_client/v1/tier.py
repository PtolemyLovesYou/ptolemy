"""Tier enum."""

from enum import StrEnum

class Tier(StrEnum):
    """Tier."""

    SYSTEM = "system"
    SUBSYSTEM = "subsystem"
    COMPONENT = "component"
    SUBCOMPONENT = "subcomponent"

    def child(self) -> "Tier":
        """Get child tier."""

        if self == Tier.SYSTEM:
            return Tier.SUBSYSTEM

        if self == Tier.SUBSYSTEM:
            return Tier.COMPONENT

        if self == Tier.COMPONENT:
            return Tier.SUBCOMPONENT

        raise ValueError("Cannot create a child trace of a subcomponent.")

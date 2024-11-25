"""Enums."""

from typing import Optional
from enum import StrEnum
from ..proto import observer_pb2 as observer


class LogType(StrEnum):
    """Log types."""

    EVENT = "event"
    RUNTIME = "runtime"
    INPUT = "input"
    OUTPUT = "output"
    FEEDBACK = "feedback"
    METADATA = "metadata"

    @classmethod
    def from_proto(
        cls, proto: observer.LogType  # pylint: disable=no-member
    ) -> "LogType":
        """
        Convert a proto LogType to a LogType enum.

        Args:
            proto (observer.LogType): The proto log type to convert.

        Returns:
            LogType: The corresponding LogType enum value.

        Raises:
            ValueError: If the proto log type is unknown.
        """
        if proto == observer.LogType.EVENT:  # pylint: disable=no-member
            return LogType.EVENT
        if proto == observer.LogType.RUNTIME:  # pylint: disable=no-member
            return LogType.RUNTIME
        if proto == observer.LogType.INPUT:  # pylint: disable=no-member
            return LogType.INPUT
        if proto == observer.LogType.OUTPUT:  # pylint: disable=no-member
            return LogType.OUTPUT
        if proto == observer.LogType.FEEDBACK:  # pylint: disable=no-member
            return LogType.FEEDBACK
        if proto == observer.LogType.METADATA:  # pylint: disable=no-member
            return LogType.METADATA

        raise ValueError(f"Unknown log type: {proto}")

    def proto(self) -> observer.LogType:  # pylint: disable=no-member
        """
        Get the proto enum value for this log type.

        Returns:
            observer.LogType: The proto enum value for this log type.

        Raises:
            ValueError: If the log type is unknown.
        """
        if self == LogType.EVENT:
            return observer.LogType.EVENT  # pylint: disable=no-member
        if self == LogType.RUNTIME:
            return observer.LogType.RUNTIME  # pylint: disable=no-member
        if self == LogType.INPUT:
            return observer.LogType.INPUT  # pylint: disable=no-member
        if self == LogType.OUTPUT:
            return observer.LogType.OUTPUT  # pylint: disable=no-member
        if self == LogType.FEEDBACK:
            return observer.LogType.FEEDBACK  # pylint: disable=no-member
        if self == LogType.METADATA:
            return observer.LogType.METADATA  # pylint: disable=no-member

        raise ValueError(f"Unknown log type: {self}")


class Tier(StrEnum):
    """Tiers."""

    SYSTEM = "system"
    SUBSYSTEM = "subsystem"
    COMPONENT = "component"
    SUBCOMPONENT = "subcomponent"

    def proto(self) -> observer.Tier:  # pylint: disable=no-member
        """
        Get the proto enum value for this tier.

        Returns:
            observer.Tier: The proto enum value for this tier.

        Raises:
            ValueError: If the tier is unknown.
        """
        if self == Tier.SYSTEM:
            return observer.Tier.SYSTEM  # pylint: disable=no-member
        if self == Tier.SUBSYSTEM:
            return observer.Tier.SUBSYSTEM  # pylint: disable=no-member
        if self == Tier.COMPONENT:
            return observer.Tier.COMPONENT  # pylint: disable=no-member
        if self == Tier.SUBCOMPONENT:
            return observer.Tier.SUBCOMPONENT  # pylint: disable=no-member

        raise ValueError(f"Unknown tier: {self}")

    @classmethod
    def from_proto(cls, proto: observer.Tier) -> "Tier":  # pylint: disable=no-member
        """
        Convert a proto Tier to a Tier enum.

        Args:
            proto (observer.Tier): The proto tier to convert.

        Returns:
            Tier: The corresponding Tier enum value.

        Raises:
            ValueError: If the proto tier is unknown.
        """
        if proto == observer.Tier.SYSTEM:  # pylint: disable=no-member
            return Tier.SYSTEM
        if proto == observer.Tier.SUBSYSTEM:  # pylint: disable=no-member
            return Tier.SUBSYSTEM
        if proto == observer.Tier.COMPONENT:  # pylint: disable=no-member
            return Tier.COMPONENT
        if proto == observer.Tier.SUBCOMPONENT:  # pylint: disable=no-member
            return Tier.SUBCOMPONENT

        raise ValueError(f"Unknown tier: {proto}")

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

"""Log objects."""

from typing import Optional

from .types import IO, ID, Tier
from .base import LogBase


class Log(LogBase): ...


class SubcomponentLog(Log):
    """Subcomponent Log."""

    TIER = Tier.SUBCOMPONENT

    component_event_id: ID

    @classmethod
    async def new(
        cls,
        component_event_id: ID,
        name: str,
        parameters: Optional[IO] = None,
    ) -> "SubcomponentLog":
        """Create a new SubcomponentLog.

        :param component_event_id: The ID of the component event that this subcomponent event belongs to.
        :param name: The name of the subcomponent event.
        :param parameters: The parameters of the subcomponent event.
        :return: A new SubcomponentLog object.
        """
        return SubcomponentLog(
            component_event_id=component_event_id,
            name=name,
            parameters=parameters,
        )


class ComponentLog(Log):
    """Component Log."""

    TIER = Tier.COMPONENT

    subsystem_event_id: ID

    @classmethod
    async def new(
        cls,
        subsystem_event_id: ID,
        name: str,
        parameters: Optional[IO] = None,
    ) -> "ComponentLog":
        """Create a new ComponentLog.

        :param subsystem_event_id: The ID of the subsystem event that this component event belongs to.
        :param name: The name of the component event.
        :param parameters: The parameters of the component event.
        :return: A new ComponentLog object.
        """
        return ComponentLog(
            subsystem_event_id=subsystem_event_id,
            name=name,
            parameters=parameters,
        )

    async def subcomponent(self, name: str, parameters: Optional[IO] = None) -> SubcomponentLog:
        """Create a new SubcomponentLog.

        :param name: The name of the subcomponent event.
        :param parameters: The parameters of the subcomponent event.
        :return: A new SubcomponentLog object.
        """
        return await SubcomponentLog.new(
            component_event_id=self.id,
            name=name,
            parameters=parameters,
        )


class SubsystemLog(Log):
    """Subsystem Log."""

    TIER = Tier.SUBSYSTEM

    system_event_id: ID

    @classmethod
    async def new(
        cls,
        system_event_id: ID,
        name: str,
        parameters: Optional[IO] = None,
    ) -> "SubsystemLog":
        """Create a new SubsystemLog.

        :param system_event_id: The ID of the system event that this subsystem event belongs to.
        :param name: The name of the subsystem event.
        :param parameters: The parameters of the subsystem event.
        :return: A new SubsystemLog object.
        """
        return SubsystemLog(
            system_event_id=system_event_id,
            name=name,
            parameters=parameters,
        )

    async def component(self, name: str, parameters: Optional[IO] = None) -> ComponentLog:
        """Create a new ComponentLog.

        :param name: The name of the component event.
        :param parameters: The parameters of the component event.
        :return: A new ComponentLog object.
        """
        return await ComponentLog.new(
            subsystem_event_id=self.id,
            name=name,
            parameters=parameters,
        )


class SystemLog(Log):
    """System log."""

    TIER = Tier.SYSTEM

    @classmethod
    async def new(
        cls,
        name: str,
        parameters: Optional[IO] = None,
    ) -> "SystemLog":
        """Create a new SystemLog.

        :param name: The name of the system event.
        :param parameters: The parameters of the system event.
        :return: A new SystemLog object.
        """
        return SystemLog(
            name=name,
            parameters=parameters,
        )

    async def subsystem(self, name: str, parameters: Optional[IO] = None) -> SubsystemLog:
        """Create a new SubsystemLog.

        :param name: The name of the subsystem event.
        :param parameters: The parameters of the subsystem event.
        :return: A new SubsystemLog object.
        """
        return await SubsystemLog.new(
            system_event_id=self.id,
            name=name,
            parameters=parameters,
        )

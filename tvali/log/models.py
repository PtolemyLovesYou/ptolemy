"""Log objects."""
from typing import Optional, ClassVar

from .types import IO, ID, Tier
from .base import _Log


class Log(_Log):
    ...


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
        return ComponentLog(
            subsystem_event_id=subsystem_event_id,
            name=name,
            parameters=parameters,
        )

    async def subcomponent(
        self,
        name: str,
        parameters: Optional[IO] = None
        ) -> SubcomponentLog:
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
        return SubsystemLog(
            system_event_id=system_event_id,
            name=name,
            parameters=parameters,
        )

    async def component(self, name: str, parameters: Optional[IO] = None) -> ComponentLog:
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
        return SystemLog(
            name=name,
            parameters=parameters,
        )

    async def subsystem(self, name: str, parameters: Optional[IO] = None) -> SubsystemLog:
        return await SubsystemLog.new(
            system_event_id=self.id,
            name=name,
            parameters=parameters,
        )

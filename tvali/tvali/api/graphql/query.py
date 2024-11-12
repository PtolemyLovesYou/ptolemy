"""GraphQL Query."""

from typing import List
import strawberry
from .types.event import (
    SystemEvent,
    SubsystemEvent,
    ComponentEvent,
    SubcomponentEvent,
    event_query_resolver,
)
from ...db import models


@strawberry.type
class Query:
    """GraphQL Query."""

    system_events: List[SystemEvent] = strawberry.field(
        resolver=event_query_resolver(SystemEvent, models.SystemEvent)
    )
    subsystem_events: List[SubsystemEvent] = strawberry.field(
        resolver=event_query_resolver(SubsystemEvent, models.SubsystemEvent)
    )
    component_events: List[ComponentEvent] = strawberry.field(
        resolver=event_query_resolver(ComponentEvent, models.ComponentEvent)
    )
    subcomponent_events: List[SubcomponentEvent] = strawberry.field(
        resolver=event_query_resolver(SubcomponentEvent, models.SubcomponentEvent)
    )

"""Router."""

from typing import List
import strawberry
from strawberry.fastapi import GraphQLRouter
from .types import (
    event_resolver_factory,
    SystemEvent,
    SubsystemEvent,
    ComponentEvent,
    SubcomponentEvent,
)
from ....utils import Tier


@strawberry.type
class Query:
    """Query."""

    system_events: List[SystemEvent] = strawberry.field(
        event_resolver_factory(Tier.SYSTEM),
        graphql_type=List[SystemEvent],
    )

    subsystem_events: List[SubsystemEvent] = strawberry.field(
        event_resolver_factory(Tier.SUBSYSTEM),
        graphql_type=List[SubsystemEvent],
    )

    component_events: List[ComponentEvent] = strawberry.field(
        event_resolver_factory(Tier.COMPONENT),
        graphql_type=List[ComponentEvent],
    )

    subcomponent_events: List[SubcomponentEvent] = strawberry.field(
        event_resolver_factory(Tier.SUBCOMPONENT),
        graphql_type=List[SubcomponentEvent],
    )

    @strawberry.field
    def health(self) -> str:
        """Return the health status as a string."""
        return "OK!"


schema = strawberry.Schema(query=Query)

router = GraphQLRouter(
    schema,
    path="/graphql",
    tags=["graphql"],
)

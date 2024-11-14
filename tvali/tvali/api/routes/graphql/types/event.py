"""Event types."""

from typing import List, Optional, ClassVar, Callable, TypeVar
from uuid import UUID
import strawberry
from .....utils import Tier, LogType
from .io import Parameters, io_resolver_factory, Input, Output, Feedback, Metadata
from .runtime import Runtime
from .runtime import runtime_resolver_factory
from ....db import models, session


@strawberry.interface
class Event:
    """Event interface."""

    TIER: ClassVar[Tier] = None

    id: UUID
    name: str
    parameters: Parameters
    environment: str
    version: str

    @strawberry.field
    def runtime(self) -> Runtime:
        return runtime_resolver_factory(self.TIER)(self)

    @strawberry.field
    def inputs(self) -> List[Input]:
        return io_resolver_factory(LogType.INPUT, self.TIER, Input)(self)

    @strawberry.field
    def outputs(self) -> List[Output]:
        return io_resolver_factory(LogType.OUTPUT, self.TIER, Output)(self)

    @strawberry.field
    def feedback(self) -> List[Feedback]:
        return io_resolver_factory(LogType.FEEDBACK, self.TIER, Feedback)(self)

    @strawberry.field
    def metadata(self) -> List[Metadata]:
        return io_resolver_factory(LogType.METADATA, self.TIER, Metadata)(self)


E = TypeVar("E", bound=Event)


def child_events_resolver(
    child_strawberry_cls: E, child_db_cls: type[models.Event], parent_tier: Tier
) -> Callable[..., List[E]]:
    """
    Generates a resolver function for fetching child events based on specified filters.

    Args:
        child_strawberry_cls (E): Strawberry type.
        child_db_cls (type[models.Event]): SQLAlchemy model class.

    Returns:
        Callable[..., List[E]]: A resolver function that retrieves a list of child events
        based on the provided filter criteria and pagination options.
    """

    def wrapper(
        parent: strawberry.Parent,
        _id: Optional[UUID] = strawberry.UNSET,
        name: Optional[str] = strawberry.UNSET,
        environment: Optional[str] = strawberry.UNSET,
        version: Optional[str] = strawberry.UNSET,
        limit: int = 10,
        offset: int = 0,
    ) -> List[child_strawberry_cls]:
        obj_filters = {
            "id": _id,
            "name": name,
            "environment": environment,
            "version": version,
            f"{parent_tier}_event_id": parent.id,
        }

        with session.get_db() as db:
            objs = (
                db.query(child_db_cls)
                .filter(
                    *[
                        getattr(child_db_cls, k) == v
                        for k, v in obj_filters.items()
                        if v is not strawberry.UNSET
                    ]
                )
                .limit(limit)
                .offset(offset)
                .all()
            )

        return [
            child_strawberry_cls(
                id=obj.id,
                name=obj.name,
                parameters=obj.parameters,
                environment=obj.environment,
                version=obj.version,
            )
            for obj in objs
        ]

    return wrapper


@strawberry.type
class SubcomponentEvent(Event):
    """Subcomponent Event"""

    TIER = Tier.SUBCOMPONENT


@strawberry.type
class ComponentEvent(Event):
    """Component Event"""

    TIER = Tier.COMPONENT
    subsystem_events = strawberry.field(
        resolver=child_events_resolver(
            SubcomponentEvent, models.SubcomponentEvent, TIER
        )
    )


@strawberry.type
class SubsystemEvent(Event):
    """Subsystem Event"""

    TIER = Tier.SUBSYSTEM
    component_events = strawberry.field(
        resolver=child_events_resolver(ComponentEvent, models.ComponentEvent, TIER)
    )


@strawberry.type
class SystemEvent(Event):
    """System event"""

    TIER = Tier.SYSTEM
    subsystem_events = strawberry.field(
        resolver=child_events_resolver(SubsystemEvent, models.SubsystemEvent, TIER)
    )


def event_query_resolver_factory(
    strawberry_cls: E, db_cls: type[models.Event]
) -> Callable[..., List[E]]:
    """
    Creates a query resolver function for fetching events from the database
    using specified filters and pagination options.

    Args:
        strawberry_cls (E): The Strawberry class representing the GraphQL type of the event.
        db_cls (type[models.Event]): The SQLAlchemy model class representing the event in the database.

    Returns:
        Callable[..., List[E]]: A resolver function that retrieves a list of events
        from the database based on the provided filter criteria and pagination options.
    """

    def wrapper(
        _id: Optional[UUID] = strawberry.UNSET,
        name: Optional[str] = strawberry.UNSET,
        environment: Optional[str] = strawberry.UNSET,
        version: Optional[str] = strawberry.UNSET,
        limit: int = 10,
        offset: int = 0,
    ) -> List[strawberry_cls]:
        obj_filters = {
            "id": _id,
            "name": name,
            "environment": environment,
            "version": version,
        }

        with session.get_db() as db:
            objs = (
                db.query(db_cls)
                .filter(
                    *[
                        getattr(db_cls, k) == v
                        for k, v in obj_filters.items()
                        if v is not strawberry.UNSET
                    ]
                )
                .limit(limit)
                .offset(offset)
                .all()
            )

        return [
            strawberry_cls(
                id=obj.id,
                name=obj.name,
                parameters=obj.parameters,
                environment=obj.environment,
                version=obj.version,
            )
            for obj in objs
        ]

    return wrapper

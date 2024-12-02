"""Strawberry types."""

from typing import Optional, List, ClassVar, Callable, Union
from datetime import datetime
import strawberry
from strawberry.scalars import JSON, ID
from sqlalchemy import select
from sqlalchemy.exc import NoResultFound
from .crud import get_objects
from .filter import EventFilter, IOFilter, MetadataFilter
from ....db import models, session
from ....utils import Tier, LogType

Timestamp = strawberry.scalar(
    datetime,
    serialize=lambda i: i.isoformat(),
    parse_value=lambda i: datetime.fromisoformat,
)


@strawberry.interface
class Record:
    """Record base class."""

    TIER: ClassVar[Tier]

    id: ID
    parent_id: ID


@strawberry.interface
class IOJSON(Record):
    """IO JSON interface."""

    field_name: str
    field_value: JSON


@strawberry.type
class Input(IOJSON):
    """Input interface."""

    LOGTYPE = LogType.INPUT


@strawberry.type
class Output(IOJSON):
    """Output interface."""

    LOGTYPE = LogType.OUTPUT


@strawberry.type
class Feedback(IOJSON):
    """Feedback interface."""

    LOGTYPE = LogType.FEEDBACK


@strawberry.type
class Metadata(Record):
    """Metadata interface."""

    LOGTYPE = LogType.METADATA

    field_name: str
    field_value: str


async def metadata_resolver(
    root: strawberry.Parent["Event"],
    filters: Optional[MetadataFilter] = None,
    limit: int = 20,
    offset: int = 0,
) -> List[Metadata]:
    """
    Resolve metadata for a given event.

    This resolver fetches metadata objects associated with the given event
    from the database, applying optional filters, limits, and offsets.

    :param root: The parent event for which metadata is to be resolved.
    :type root: strawberry.Parent["Event"]
    :param filters: Optional filters to apply when fetching metadata.
    :type filters: Optional[MetadataFilter]
    :param limit: The maximum number of metadata objects to return, default is 20.
    :type limit: int
    :param offset: The number of metadata objects to skip before starting to collect the result set.
    :type offset: int
    :return: A list of Metadata objects associated with the event.
    :rtype: List[Metadata]
    """
    model = models.DB_OBJ_MAP[LogType.METADATA][root.TIER]

    objs: List[models.EventMetadata] = await get_objects(
        model, filters=filters, limit=limit, offset=offset, parent_id=root.id
    )

    return [
        Metadata(
            id=obj.id,
            parent_id=obj.parent_id,
            field_name=obj.field_name,
            field_value=obj.field_value,
        )
        for obj in objs
    ]


def io_resolver_factory(log_type: LogType) -> List[IOJSON]:
    """
    Creates an IO resolver for a specific log type.

    This function returns an asynchronous resolver that fetches IO objects
    (Input, Output, or Feedback) associated with a given event from the database
    according to the specified log type. The resolver supports filtering,
    limiting, and offsetting of the results.

    :param log_type: The type of IO log to resolve (Input, Output, or Feedback).
    :type log_type: LogType
    :return: An asynchronous resolver function that returns a list of IOJSON objects.
    :rtype: Callable[[strawberry.Parent["Event"], Optional[IOFilter], Optional[int], Optional[int]], List[IOJSON]]
    :raises ValueError: If an unsupported log type is provided.
    """

    async def io_resolver(
        root: strawberry.Parent["Event"],
        filters: Optional[IOFilter] = None,
        limit: Optional[int] = None,
        offset: Optional[int] = None,
    ) -> List[IOJSON]:
        model = models.DB_OBJ_MAP[log_type][root.TIER]

        objs: list[
            Union[models.EventInput, models.EventOutput, models.EventFeedback]
        ] = await get_objects(
            model, filters=filters, limit=limit, offset=offset, parent_id=root.id
        )

        if log_type == LogType.INPUT:
            log_cls = Input
        elif log_type == LogType.OUTPUT:
            log_cls = Output
        elif log_type == LogType.FEEDBACK:
            log_cls = Feedback
        else:
            raise ValueError(f"Must use Input, Output, or Feedback, not {log_type}")

        return [
            log_cls(
                id=obj.id,
                parent_id=obj.parent_id,
                field_name=obj.field_name,
                field_value=obj.field_value,
            )
            for obj in objs
        ]

    return io_resolver


@strawberry.type
class Runtime(Record):
    """Runtime interface."""

    LOGTYPE = LogType.RUNTIME

    start_time: Optional[Timestamp] = None  # type: ignore
    end_time: Optional[Timestamp] = None  # type: ignore
    error_type: Optional[str] = None
    error_content: Optional[str] = None


@strawberry.interface
class Event(Record):
    """Event interface."""

    TIER: ClassVar[Tier]

    name: str
    parameters: Optional[JSON] = None
    environment: Optional[str] = None
    version: Optional[str] = None

    @strawberry.field
    async def runtime(self) -> Optional[Runtime]:
        """Get the runtime for this event."""
        model = models.DB_OBJ_MAP[LogType.RUNTIME][self.TIER]
        try:
            obj: models.EventRuntime = await get_objects(
                model, parent_id=self.id, mode="one", limit=1
            )
        except NoResultFound:
            return None

        return Runtime(
            id=obj.id,
            parent_id=obj.parent_id,
            start_time=obj.start_time,
            end_time=obj.end_time,
            error_type=obj.error_type,
            error_content=obj.error_content,
        )

    inputs = strawberry.field(
        resolver=io_resolver_factory(LogType.INPUT), graphql_type=List[Input]
    )
    outputs = strawberry.field(
        resolver=io_resolver_factory(LogType.OUTPUT), graphql_type=List[Output]
    )
    feedback = strawberry.field(
        resolver=io_resolver_factory(LogType.FEEDBACK), graphql_type=List[Feedback]
    )
    metadata = strawberry.field(resolver=metadata_resolver, graphql_type=List[Metadata])


def children_resolver_factory(child_type: Event) -> Callable[[Event], List[Event]]:
    """
    Generate a resolver for fetching child events of a given event.

    This factory function takes in the type of the child event and returns a
    resolver function that can be used in strawberry to fetch the child events
    for a given event. The resolver takes in the parent event, optional filters,
    limits, and offsets, and returns a list of child events.
    """

    async def children_resolver(
        root: strawberry.Parent[Event],
        filters: Optional[EventFilter] = None,
        limit: Optional[int] = None,
        offset: Optional[int] = None,
    ) -> List[Event]:
        model = models.DB_OBJ_MAP[LogType.EVENT][root.TIER.child]

        objs: List[models.Event] = await get_objects(
            model, filters=filters, limit=limit, offset=offset, parent_id=root.id
        )

        return [
            child_type(
                id=obj.id,
                parent_id=obj.parent_id,
                name=obj.name,
                parameters=obj.parameters,
                environment=obj.environment,
                version=obj.version,
            )
            for obj in objs
        ]

    return children_resolver


@strawberry.type
class SubcomponentEvent(Event):
    """Subcomponent event."""

    TIER = Tier.SUBCOMPONENT


@strawberry.type
class ComponentEvent(Event):
    """Component event."""

    TIER = Tier.COMPONENT

    subcomponent_events = strawberry.field(
        children_resolver_factory(SubcomponentEvent),
        graphql_type=List[SubcomponentEvent],
    )


@strawberry.type
class SubsystemEvent(Event):
    """Subsystem event."""

    TIER = Tier.SUBSYSTEM

    component_events = strawberry.field(
        children_resolver_factory(ComponentEvent),
        graphql_type=List[ComponentEvent],
    )


@strawberry.type
class SystemEvent(Event):
    """System event."""

    TIER = Tier.SYSTEM

    subsystem_events = strawberry.field(
        children_resolver_factory(SubsystemEvent),
        graphql_type=List[SubsystemEvent],
    )


STRAWBERRY_TIER_MAP = {
    Tier.SYSTEM: SystemEvent,
    Tier.SUBSYSTEM: SubsystemEvent,
    Tier.COMPONENT: ComponentEvent,
    Tier.SUBCOMPONENT: SubcomponentEvent,
}


def event_resolver_factory(tier: Tier) -> Callable[..., List[Event]]:
    """
    Get a resolver for the given tier's events.

    :param tier: The tier of events to get.
    :type tier: Tier
    :return: A resolver that returns a list of Strawberry objects.
    :rtype: Callable[..., List[E]]
    """

    async def event_resolver(
        filters: Optional[EventFilter] = None,
        limit: Optional[int] = None,
        offset: Optional[int] = None,
        parent_id: Optional[str] = None,
    ) -> List[Event]:
        model = models.DB_OBJ_MAP[LogType.EVENT][tier]

        query = select(model).order_by(model.created_at.asc())

        if limit:
            query = query.limit(limit)
        if offset:
            query = query.offset(offset)

        filters_ = [
            getattr(model, i) == j
            for i, j in filters.__dict__.items()
            if j != strawberry.UNSET
        ]

        if parent_id:
            filters_ += [model.parent_id == parent_id]

        if filters:
            query = query.filter(*filters_)

        async with session.get_db() as db:
            result = await db.execute(query)

            objs: List[models.Event] = result.scalars().all()

        return [
            STRAWBERRY_TIER_MAP[tier](
                id=obj.id,
                parent_id=obj.parent_id,
                name=obj.name,
                parameters=obj.parameters,
                environment=obj.environment,
                version=obj.version,
            )
            for obj in objs
        ]

    return event_resolver

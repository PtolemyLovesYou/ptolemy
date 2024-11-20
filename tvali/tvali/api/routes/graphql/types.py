"""Strawberry types."""

from typing import Optional, List, ClassVar, Callable
from datetime import datetime
import strawberry
from strawberry.scalars import JSON, ID
from sqlalchemy import select
from sqlalchemy.exc import NoResultFound
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
    offset: int = 0
) -> List[Metadata]:
    model = models.DB_OBJ_MAP[LogType.METADATA][root.TIER]

    query = (
        select(model)
        .limit(limit)
        .offset(offset)
        .order_by(model.created_at.asc())
        )

    filters_ = [model.parent_id == root.id]

    if filters:
        filters_ += [
            getattr(model, k) == v
            for k, v in filters.__dict__.items()
            if v != strawberry.UNSET
        ]

        query = query.filter(*filters_)

    async with session.get_db() as db:
        result = await db.execute(query)

    objs = result.scalars().all()

    return [
        Metadata(
            id=obj.id,
            parent_id=obj.parent_id,
            field_name=obj.field_name,
            field_value=obj.field_value,
        )
        for obj in objs
    ]

def get_io_resolver(log_type: LogType) -> List[IOJSON]:
    async def wrapper(
        root: strawberry.Parent["Event"],
        filters: Optional[IOFilter] = None,
        limit: int = 20,
        offset: int = 0
    ) -> List[IOJSON]:
        model = models.DB_OBJ_MAP[log_type][root.TIER]

        query = select(model).limit(limit).offset(offset).order_by(model.created_at.asc())

        filters_ = [model.parent_id == root.id]

        if filters:
            filters_ += [
                getattr(model, k) == v
                for k, v in filters.__dict__.items()
                if v != strawberry.UNSET
            ]

            query = query.filter(*filters_)

        async with session.get_db() as db:
            result = await db.execute(query)

        objs = result.scalars().all()

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

    return wrapper


@strawberry.type
class Runtime(Record):
    """Runtime interface."""

    LOGTYPE = LogType.RUNTIME

    start_time: Timestamp  # type: ignore
    end_time: Timestamp  # type: ignore
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
        async with session.get_db() as db:
            result = await db.execute(select(model).where(model.parent_id == self.id))

        try:
            obj = result.scalars().one()
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
        resolver=get_io_resolver(LogType.INPUT), graphql_type=List[Input]
    )
    outputs = strawberry.field(
        resolver=get_io_resolver(LogType.OUTPUT), graphql_type=List[Output]
    )
    feedback = strawberry.field(
        resolver=get_io_resolver(LogType.FEEDBACK), graphql_type=List[Feedback]
    )
    metadata = strawberry.field(
        resolver=metadata_resolver, graphql_type=List[Metadata]
    )


def get_children_resolver(child_type: Event) -> Callable[[Event], List[Event]]:
    async def resolver(
        root: strawberry.Parent[Event],
        filters: Optional[EventFilter] = None,
        limit: int = 20,
        offset: int = 0
        ) -> List[Event]:
        model = models.DB_OBJ_MAP[LogType.EVENT][root.TIER.child]

        query = select(model).limit(limit).offset(offset).order_by(model.created_at.asc())

        if filters:
            filters = [
                getattr(model, i) == j
                for i, j in filters.__dict__.items()
                if j != strawberry.UNSET
            ] + [model.parent_id == root.id]

            query = query.filter(*filters)

        async with session.get_db() as db:
            result = await db.execute(select(model).filter(model.parent_id == root.id))

        objs = result.scalars().all()

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

    return resolver


@strawberry.type
class SubcomponentEvent(Event):
    """Subcomponent event."""

    TIER = Tier.SUBCOMPONENT


@strawberry.type
class ComponentEvent(Event):
    """Component event."""

    TIER = Tier.COMPONENT

    subcomponent_events = strawberry.field(
        get_children_resolver(SubcomponentEvent),
        graphql_type=List[SubcomponentEvent],
    )


@strawberry.type
class SubsystemEvent(Event):
    """Subsystem event."""

    TIER = Tier.SUBSYSTEM

    component_events = strawberry.field(
        get_children_resolver(ComponentEvent),
        graphql_type=List[ComponentEvent],
    )


@strawberry.type
class SystemEvent(Event):
    """System event."""

    TIER = Tier.SYSTEM

    subsystem_events = strawberry.field(
        get_children_resolver(SubsystemEvent),
        graphql_type=List[SubsystemEvent],
    )


STRAWBERRY_TIER_MAP = {
    Tier.SYSTEM: SystemEvent,
    Tier.SUBSYSTEM: SubsystemEvent,
    Tier.COMPONENT: ComponentEvent,
    Tier.SUBCOMPONENT: SubcomponentEvent,
}


def get_event_resolver(tier: Tier) -> Callable[..., List[Event]]:
    """
    Get a resolver for the given tier's events.

    :param tier: The tier of events to get.
    :type tier: Tier
    :return: A resolver that returns a list of Strawberry objects.
    :rtype: Callable[..., List[E]]
    """

    async def resolver(filters: Optional[EventFilter] = None, limit: int = 20, offset: int = 0) -> List[Event]:
        model = models.DB_OBJ_MAP[LogType.EVENT][tier]

        query = select(model).limit(limit).offset(offset).order_by(model.created_at.asc())

        if filters:
            query = (
                query.filter(
                *[
                    getattr(model, i) == j
                    for i, j in filters.__dict__.items()
                    if j != strawberry.UNSET
                    ]
                )
            )

        async with session.get_db() as db:
            result = await db.execute(query)

            objs = result.scalars().all()

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

    return resolver

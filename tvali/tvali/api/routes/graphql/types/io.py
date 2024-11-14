"""IO type."""

from typing import NewType, Generic, List, Callable, TypeVar
from uuid import UUID
import strawberry
from sqlalchemy import select
from .....utils import LogType, Tier
from ....db import models, session

T = TypeVar("T")

JSON = strawberry.scalar(
    NewType("JSON", object),
    description="The `JSON` scalar type represents JSON values as specified by ECMA-404",
    serialize=lambda v: v,
    parse_value=lambda v: v,
)

Parameters = strawberry.scalar(
    NewType("Parameters", dict),
    description="System parameters. Dict[str, Any].",
    serialize=lambda v: v,
    parse_value=lambda v: v,
)


@strawberry.type
class IO(Generic[T]):
    """IO strawberry type."""

    id: UUID
    field_name: str
    field_value: T


@strawberry.type
class Input(IO[JSON]):
    """Input type."""


@strawberry.type
class Output(IO[JSON]):
    """Output type."""


@strawberry.type
class Feedback(IO[JSON]):
    """Feedback type."""


@strawberry.type
class Metadata(IO[str]):
    """Metadata type."""


IOType = TypeVar("IOType", bound=IO)  # pylint: disable=invalid-name


def io_resolver_factory(
    log_type: LogType, tier: Tier, io_type: IOType
) -> Callable[[strawberry.Parent], List[IO]]:
    """Get IO resolver."""

    async def wrapper(parent: strawberry.Parent) -> List[io_type]:
        model = models.DB_OBJ_MAP[log_type][tier]
        async with session.get_db() as db:
            result = await db.execute(select(model).where(model.parent_id == parent.id))

        objs = result.scalars().all()

        return [
            io_type(id=obj.id, field_name=obj.field_name, field_value=obj.field_value)
            for obj in objs
        ]

    return wrapper

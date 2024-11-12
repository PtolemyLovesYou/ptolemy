"""IO type."""

from typing import NewType, Generic, List, Callable
from uuid import UUID
import strawberry
from ....db import models, session
from ....utils.types import T
from ....utils.enums import LogType, Tier

JSON = strawberry.scalar(
    NewType("JSON", object),
    description="The `JSON` scalar type represents JSON values as specified by ECMA-404",
    serialize=lambda v: v,
    parse_value=lambda v: v,
)


@strawberry.type
class IO(Generic[T]):
    """IO strawberry type."""

    id: UUID
    field_name: str
    field_value: T


def io_resolver_factory(
    log_type: LogType, tier: Tier, io_type: T
) -> Callable[[strawberry.Parent], List[IO[T]]]:
    """Get IO resolver."""

    def wrapper(parent: strawberry.Parent) -> List[IO[io_type]]:
        model = models.DB_OBJ_MAP[log_type][tier]
        with session.get_db() as db:
            objs = db.query(model).filter(model.parent_id == parent.id).all()

        io_type = str if log_type == LogType.RUNTIME else JSON
        return [
            IO[io_type](
                id=obj.id, field_name=obj.field_name, field_value=obj.field_value
            )
            for obj in objs
        ]

    return wrapper

"""Runtime type."""

from typing import Optional, Callable
from uuid import UUID
from datetime import datetime
import strawberry
from sqlalchemy import select
from .....utils import LogType, Tier
from ....db import models, session


@strawberry.type
class Runtime:
    """Runtime class."""

    id: UUID
    start_time: datetime
    end_time: datetime
    error_type: Optional[str]
    error_content: Optional[str]

    @strawberry.field
    def status(self) -> str:
        """
        Determine the status of the runtime based on the presence of errors.

        Returns:
            str: Returns "ERROR" if there is an error type or error content;
            otherwise, returns "SUCCESS".
        """
        if self.error_type or self.error_content:
            return "ERROR"

        return "SUCCESS"


def runtime_resolver_factory(tier: Tier) -> Callable[[strawberry.Parent], Runtime]:
    """Get runtime resolver."""

    async def wrapper(parent: strawberry.Parent) -> Runtime:
        model: models.EventRuntime = models.DB_OBJ_MAP[LogType.RUNTIME][tier]
        async with session.get_db() as db:
            result = await db.execute(
                select(model).where(model.parent_id == parent.id)
            )

            obj = result.scalars().one()

        return Runtime(
            id=obj.id,
            start_time=obj.start_time,
            end_time=obj.end_time,
            error_type=obj.error_type,
            error_content=obj.error_content,
        )

    return wrapper

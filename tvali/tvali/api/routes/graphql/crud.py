"""CRUD operations."""

from typing import Optional, TypeVar, Literal, List
from sqlalchemy import select
import strawberry
from .filter import QueryFilter
from ....db import models, session

ET = TypeVar("ET", bound=models.EventTable)


async def get_objects(
    model: type[ET],
    filters: Optional[QueryFilter] = None,
    limit: Optional[int] = None,
    offset: Optional[int] = None,
    parent_id: Optional[str] = None,
    mode: Literal["all", "one"] = "all",
) -> List[ET]:
    """
    Get objects from the database.

    :param model: The SQLAlchemy model to query.
    :param filters: Optional filters to apply to the query.
    :param limit: Optional limit to the number of objects to return.
    :param offset: Optional offset from the start of the query.
    :param parent_id: Optional parent ID to filter by.
    :param mode: Optional mode to return either all objects or one object.
    :return: A list of objects if `mode` is 'all', otherwise a single object.
    """
    query = select(model).order_by(model.created_at.asc())

    if limit:
        query = query.limit(limit)
    if offset:
        query = query.offset(offset)

    filters_ = []

    if filters:
        filters_ += [
            getattr(model, i) == j
            for i, j in filters.__dict__.items()
            if j != strawberry.UNSET
        ]

    if parent_id:
        filters_.append(model.parent_id == parent_id)

    if filters_:
        query = query.filter(*filters_)

    async with session.get_db() as db:
        result = await db.execute(query)

    if mode == "one":
        return result.scalars().one()
    elif mode == "all":
        return result.scalars().all()
    else:
        raise ValueError(f"Invalid mode: {mode}")

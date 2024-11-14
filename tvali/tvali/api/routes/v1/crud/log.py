"""CRUD ops for logs."""

import logging
from typing import Callable, List, Annotated
from pydantic import BaseModel
from sqlalchemy.exc import SQLAlchemyError, NoResultFound
from fastapi import HTTPException, Query
from ..schemas.log import Log, CreateSchema, RecordSchema, QueryMixin
from .....utils import Tier, LogType
from ....db import session, models

logger = logging.getLogger(__name__)


class LogCRUDFactory(BaseModel):
    """Log CRUD method factory."""

    tier: Tier
    log_type: LogType

    @property
    def db_class(self) -> type[models.EventTable]:
        """Database class."""
        return models.DB_OBJ_MAP[self.log_type][self.tier]

    def create_function(self) -> Callable[[List[Log]], List[dict[str, str]]]:
        """Generate create endpoint for object."""

        async def create(
            data: List[Log[self.tier, self.log_type, CreateSchema]]
        ) -> List[dict[str, str]]:
            with session.get_db() as db:
                try:
                    objs = [
                        self.db_class(**d.model_dump(exclude_none=True)) for d in data
                    ]
                    db.add_all(objs)
                    db.commit()
                except SQLAlchemyError as e:
                    db.rollback()
                    logger.error("Database error in create_event: %s", e)
                    raise HTTPException(
                        status_code=500,
                        detail="Database error in create_event",
                    ) from e

            return [{"id": str(obj.id)} for obj in objs]

        return create

    def get_function(self) -> Callable[[str], Log]:
        """Generate get endpoint for object."""

        async def get(
            query: Annotated[Log[self.tier, self.log_type, QueryMixin], Query()]
        ) -> List[Log[self.tier, self.log_type, RecordSchema]]:
            query_params = query.model_dump(
                exclude_none=True, exclude=["order_by", "limit", "offset"]
            )
            filter_params = [
                getattr(self.db_class, k) == v for k, v in query_params.items()
            ]

            with session.get_db() as db:
                try:
                    print(self.db_class, filter_params, query)
                    objs = (
                        db
                        .query(self.db_class)
                        .filter(*filter_params)
                        .limit(query.limit)
                        .offset(query.offset)
                        .all()
                    )
                except SQLAlchemyError as e:
                    logger.error("Database error in get_event: %s", e)
                    raise HTTPException(
                        status_code=500,
                        detail="Database error in get_event",
                    ) from e

            return [
                Log[self.tier, self.log_type, RecordSchema].model_validate(obj.__dict__)
                for obj in objs
            ]

        return get

    def delete_function(self) -> Callable[[str], dict[str, str]]:
        """Generate delete endpoint for object."""

        async def delete(id_: str) -> dict[str, str]:
            with session.get_db() as db:
                try:
                    item = db.query(self.db_class).filter(self.db_class.id == id_).first()

                    if not item:
                        raise HTTPException(
                            status_code=404,
                            detail=f"Could not find {self.tier.capitalize()}{self.log_type.capitalize()} object with id {id_}"
                        )
                    db.delete(item)

                    db.commit()
                except NoResultFound as e:
                    db.rollback()
                    raise HTTPException(
                        status_code=404,
                        detail="Could not find object to delete"
                    ) from e
                except SQLAlchemyError as e:
                    db.rollback()
                    logger.error("Database error in delete_event: %s", e)
                    raise HTTPException(
                        status_code=500,
                        detail="Database error in delete_event",
                    ) from e
                return {"status": "success"}

        return delete

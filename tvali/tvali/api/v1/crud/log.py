"""CRUD ops for logs."""

import logging
from typing import Callable
from pydantic import BaseModel
from sqlalchemy.exc import SQLAlchemyError
from fastapi import HTTPException
from ..schemas.log import Log, CreateSchema, RecordSchema
from ....db import session, models
from ....utils.enums import Tier, LogType

logger = logging.getLogger(__name__)


class LogCRUDFactory(BaseModel):
    """Log CRUD method factory."""

    tier: Tier
    log_type: LogType

    @property
    def db_class(self) -> type[models.EventTable]:
        """Database class."""
        return models.DB_OBJ_MAP[self.log_type][self.tier]

    def create_function(self) -> Callable[[Log], dict[str, str]]:
        """Generate create endpoint for object."""

        async def create(
            data: Log[self.tier, self.log_type, CreateSchema]
        ) -> dict[str, str]:
            with session.get_db() as db:
                try:
                    obj = self.db_class(**data.model_dump(exclude_none=True))
                    db.add(obj)
                    db.commit()
                except SQLAlchemyError as e:
                    db.rollback()
                    logger.error("Database error in create_event: %s", e)
                    raise HTTPException(
                        status_code=500,
                        detail="Database error in create_event",
                    ) from e

            return {"id": obj.id.hex}

        return create

    def get_function(self) -> Callable[[str], Log]:
        """Generate get endpoint for object."""

        async def get(id_: str) -> Log[self.tier, self.log_type, RecordSchema]:
            with session.get_db() as db:
                try:
                    obj = (
                        db.query(self.db_class).filter(self.db_class.id == id_).first()
                    )
                except SQLAlchemyError as e:
                    logger.error("Database error in get_event: %s", e)
                    raise HTTPException(
                        status_code=500,
                        detail="Database error in get_event",
                    ) from e

            return Log[self.tier, self.log_type, RecordSchema].model_validate(
                obj.__dict__
            )

        return get

    def delete_function(self) -> Callable[[str], dict[str, str]]:
        """Generate delete endpoint for object."""

        async def delete(id_: str) -> dict[str, str]:
            with session.get_db() as db:
                db.query(self.db_class).filter(self.db_class.id == id_).delete()

                db.commit()
                return {"status": "success"}

        return delete

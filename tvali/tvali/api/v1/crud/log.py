"""CRUD ops for logs."""
import logging
from typing import Callable, Dict
from pydantic import BaseModel
from sqlalchemy.exc import SQLAlchemyError
from fastapi import HTTPException
from ..schemas.log import LogSchema, Create, Record
from ....db import models, session
from ....utils.enums import Tier, LogType

logger = logging.getLogger(__name__)

DB_OBJ_MAP : Dict[LogType, Dict[Tier, type[models.EventTable]]]= {
    LogType.EVENT: {
        Tier.SYSTEM: models.SystemEvent,
        Tier.SUBSYSTEM: models.SubsystemEvent,
        Tier.COMPONENT: models.ComponentEvent,
        Tier.SUBCOMPONENT: models.SubcomponentEvent,
    },
    LogType.RUNTIME: {
        Tier.SYSTEM: models.SystemRuntime,
        Tier.SUBSYSTEM: models.SubsystemRuntime,
        Tier.COMPONENT: models.ComponentRuntime,
        Tier.SUBCOMPONENT: models.SubcomponentRuntime,
        },
    LogType.INPUT: {
        Tier.SYSTEM: models.SystemInput,
        Tier.SUBSYSTEM: models.SubsystemInput,
        Tier.COMPONENT: models.ComponentInput,
        Tier.SUBCOMPONENT: models.SubcomponentInput,
        },
    LogType.OUTPUT: {
        Tier.SYSTEM: models.SystemOutput,
        Tier.SUBSYSTEM: models.SubsystemOutput,
        Tier.COMPONENT: models.ComponentOutput,
        Tier.SUBCOMPONENT: models.SubcomponentOutput,
        },
    LogType.FEEDBACK: {
        Tier.SYSTEM: models.SystemFeedback,
        Tier.SUBSYSTEM: models.SubsystemFeedback,
        Tier.COMPONENT: models.ComponentFeedback,
        Tier.SUBCOMPONENT: models.SubcomponentFeedback,
        },
    LogType.METADATA: {
        Tier.SYSTEM: models.SystemMetadata,
        Tier.SUBSYSTEM: models.SubsystemMetadata,
        Tier.COMPONENT: models.ComponentMetadata,
        Tier.SUBCOMPONENT: models.SubcomponentMetadata,
        },
    }

class LogCRUDFactory(BaseModel):
    """Log CRUD method factory."""
    tier: Tier
    log_type: LogType

    @property
    def db_class(self) -> type[models.EventTable]:
        """Database class."""
        return DB_OBJ_MAP[self.log_type][self.tier]

    def create_function(self) -> Callable[[LogSchema], dict[str, str]]:
        """Generate create endpoint for object."""
        async def create(data: LogSchema[self.tier, self.log_type, Create]) -> dict[str, str]:
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

    def get_function(self) -> Callable[[str], LogSchema]:
        """Generate get endpoint for object."""
        async def get(id_: str) -> LogSchema[self.tier, self.log_type, Record]:
            with session.get_db() as db:
                try:
                    obj = db.query(self.db_class).filter(self.db_class.id == id_).first()
                except SQLAlchemyError as e:
                    logger.error("Database error in get_event: %s", e)
                    raise HTTPException(
                        status_code=500,
                        detail="Database error in get_event",
                    ) from e

            return LogSchema[self.tier, self.log_type, Record].model_validate(obj.__dict__)

        return get

    def delete_function(self) -> Callable[[str], dict[str, str]]:
        """Generate delete endpoint for object."""
        async def delete(id_: str) -> dict[str, str]:
            with session.get_db() as db:
                db.query(self.db_class).filter(self.db_class.id == id_).delete()

                db.commit()
                return {"status": "success"}

        return delete

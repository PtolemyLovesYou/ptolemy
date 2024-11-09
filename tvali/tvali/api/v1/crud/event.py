"""CRUD operations for events."""

from uuid import UUID
from ....db import models, session
from ..schemas.core import Record, Create


def get_event(
    db_class: type[models.EventTable], return_class: type[Record], idx: str
) -> Record:
    """
    Get event.

    Args:
        db_class (type[models.EventTable]): Database class.
        return_class (type[Record]): Return class.
        idx (str): ID.

    Returns:
        Record: Event.
    """
    db = session.SessionLocal()
    event = db.query(db_class).filter(db_class.id == idx).first()
    return return_class(**event.__dict__)

def create_event(
    data: Create, db_class: type[models.EventTable]
) -> dict[str, UUID]:
    """
    Create event.

    Args:
        data (Create): Data create schema.
        db_class (type[models.EventTable]): Database class.

    Returns:
        UUID: Object ID.
    """
    db = session.SessionLocal()
    obj = db_class(**data.model_dump())
    db.add(obj)
    db.commit()
    db.refresh(obj)

    return {"id": obj.id}

def delete_event(db_class: type[models.EventTable], idx: str) -> dict[str, str]:
    """
    Delete event.

    Args:
        db_class (type[models.EventTable]): Database class.
        idx (str): ID.

    Returns:
        dict[str, str]: Status.
    """
    db = session.SessionLocal()
    db.query(db_class).filter(db_class.id == idx).delete()
    db.commit()

    return {"status": "success"}

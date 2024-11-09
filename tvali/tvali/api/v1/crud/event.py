"""CRUD operations for events."""
from ....db import models, session
from ..schemas.core import Record

def get_event(
    db_class: type[models.EventTable],
    return_class: type[Record],
    idx: str
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

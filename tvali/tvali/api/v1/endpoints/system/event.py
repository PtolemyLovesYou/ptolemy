"""System event endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event
from ...schemas.system import SystemEvent, SystemEventCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/event",
    tags=["event"],
)


@router.get("/{system_event_id}")
async def get_system_event(system_event_id: str) -> SystemEvent:
    """Get system event."""
    return get_event(models.SystemEvent, SystemEvent, system_event_id)


@router.post("/")
async def create_system_event(event: SystemEventCreate):
    """Create system event."""
    db = SessionLocal()
    db_event = models.SystemEvent(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{system_event_id}")
async def delete_system_event(system_event_id: str):
    """Delete system event."""
    db = SessionLocal()
    db.query(models.SystemEvent).filter(
        models.SystemEvent.id == system_event_id
    ).delete()
    db.commit()

    return {"status": "success"}

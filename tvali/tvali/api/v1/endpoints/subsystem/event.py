"""Subsystem event endpoints."""
from fastapi import APIRouter
from ...schemas.subsystem import SubsystemEvent, SubsystemEventCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/event",
    tags=["event"],
)

@router.get("/{subsystem_event_id}")
async def get_subsystem_event(subsystem_event_id: str) -> SubsystemEvent:
    """Get subsystem event."""
    db = SessionLocal()
    event = db.query(models.SubsystemEvent).filter(models.SubsystemEvent.id == subsystem_event_id).first()
    return event

@router.post("/")
async def create_subsystem_event(event: SubsystemEventCreate):
    """Create subsystem event."""
    db = SessionLocal()
    db_event = models.SubsystemEvent(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {
        "id": db_event.id
    }

@router.delete("/{subsystem_event_id}")
async def delete_subsystem_event(subsystem_event_id: str):
    """Delete subsystem event."""
    db = SessionLocal()
    db.query(models.SubsystemEvent).filter(models.SubsystemEvent.id == subsystem_event_id).delete()
    db.commit()

    return {
        "status": "success"
    }

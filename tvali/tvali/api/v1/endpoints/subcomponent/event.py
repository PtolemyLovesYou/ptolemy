"""Subcomponent event endpoints."""
from fastapi import APIRouter
from ...schemas.subcomponent import SubcomponentEvent, SubcomponentEventCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/event",
    tags=["event"],
)

@router.get("/{subcomponent_event_id}")
async def get_subcomponent_event(subcomponent_event_id: str) -> SubcomponentEvent:
    """Get subcomponent event."""
    db = SessionLocal()
    event = db.query(models.SubcomponentEvent).filter(models.SubcomponentEvent.id == subcomponent_event_id).first()
    return event

@router.post("/")
async def create_subcomponent_event(event: SubcomponentEventCreate):
    """Create subcomponent event."""
    db = SessionLocal()
    db_event = models.SubcomponentEvent(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {
        "id": db_event.id
    }

@router.delete("/{subcomponent_event_id}")
async def delete_subcomponent_event(subcomponent_event_id: str):
    """Delete subcomponent event."""
    db = SessionLocal()
    db.query(models.SubcomponentEvent).filter(models.SubcomponentEvent.id == subcomponent_event_id).delete()
    db.commit()

    return {
        "status": "success"
    }

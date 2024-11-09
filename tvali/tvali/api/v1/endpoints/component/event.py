"""Component event endpoints."""
from fastapi import APIRouter
from ...schemas.component import ComponentEvent, ComponentEventCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/event",
    tags=["event"],
)

@router.get("/{component_event_id}")
async def get_component_event(component_event_id: str) -> ComponentEvent:
    """Get component event."""
    db = SessionLocal()
    event = db.query(models.ComponentEvent).filter(models.ComponentEvent.id == component_event_id).first()
    return event

@router.post("/")
async def create_component_event(event: ComponentEventCreate):
    """Create component event."""
    db = SessionLocal()
    db_event = models.ComponentEvent(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {
        "id": db_event.id
    }

@router.delete("/{component_event_id}")
async def delete_component_event(component_event_id: str):
    """Delete component event."""
    db = SessionLocal()
    db.query(models.ComponentEvent).filter(models.ComponentEvent.id == component_event_id).delete()
    db.commit()

    return {
        "status": "success"
    }

"""Component input endpoints."""
from fastapi import APIRouter
from ...schemas.component import ComponentInput, ComponentInputCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/input",
    tags=["input"],
)

@router.get("/{component_input_id}")
async def get_component_input(component_input_id: str) -> ComponentInput:
    """Get component input."""
    db = SessionLocal()
    event = (
        db
        .query(models.ComponentInput)
        .filter(models.ComponentEvent.id == component_input_id)
        .first()
        )

    return event

@router.post("/")
async def create_component_input(event: ComponentInputCreate):
    """Create component input."""
    db = SessionLocal()
    db_event = models.ComponentInput(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {
        "id": db_event.id
    }

@router.delete("/{component_input_id}")
async def delete_component_input(component_input_id: str):
    """Delete component input."""
    db = SessionLocal()
    db.query(models.ComponentInput).filter(models.ComponentEvent.id == component_input_id).delete()
    db.commit()

    return {
        "status": "success"
    }

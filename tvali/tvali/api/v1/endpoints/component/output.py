"""Component output endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event
from ...schemas.component import ComponentOutput, ComponentOutputCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/output",
    tags=["output"],
)


@router.get("/{component_output_id}")
async def get_component_output(component_output_id: str) -> ComponentOutput:
    """Get component output."""
    return get_event(models.ComponentOutput, ComponentOutput, component_output_id)


@router.post("/")
async def create_component_output(event: ComponentOutputCreate):
    """Create component output."""
    db = SessionLocal()
    db_event = models.ComponentOutput(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{component_output_id}")
async def delete_component_output(component_output_id: str):
    """Delete component output."""
    db = SessionLocal()
    db.query(models.ComponentOutput).filter(
        models.ComponentEvent.id == component_output_id
    ).delete()
    db.commit()

    return {"status": "success"}

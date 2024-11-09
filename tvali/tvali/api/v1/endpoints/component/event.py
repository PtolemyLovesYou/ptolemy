"""Component event endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.component import ComponentEvent, ComponentEventCreate
from .....db import models

router = APIRouter(
    prefix="/event",
    tags=["event"],
)


@router.get("/{component_event_id}")
async def get_component_event(component_event_id: str) -> ComponentEvent:
    """Get component event."""
    return get_event(models.ComponentEvent, ComponentEvent, component_event_id)


@router.post("/")
async def create_component_event(event: ComponentEventCreate):
    """Create component event."""
    return create_event(event, models.ComponentEvent)


@router.delete("/{component_event_id}")
async def delete_component_event(component_event_id: str):
    """Delete component event."""
    return delete_event(models.ComponentEvent, component_event_id)

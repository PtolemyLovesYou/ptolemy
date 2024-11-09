"""Component input endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.component import ComponentInput, ComponentInputCreate
from .....db import models

router = APIRouter(
    prefix="/input",
    tags=["input"],
)


@router.get("/{component_input_id}")
async def get_component_input(component_input_id: str) -> ComponentInput:
    """Get component input."""
    return get_event(models.ComponentInput, ComponentInput, component_input_id)


@router.post("/")
async def create_component_input(event: ComponentInputCreate):
    """Create component input."""
    return create_event(event, models.ComponentInput)


@router.delete("/{component_input_id}")
async def delete_component_input(component_input_id: str):
    """Delete component input."""
    return delete_event(models.ComponentInput, component_input_id)

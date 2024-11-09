"""Component output endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.component import ComponentOutput, ComponentOutputCreate
from .....db import models

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
    return create_event(event, models.ComponentOutput)


@router.delete("/{component_output_id}")
async def delete_component_output(component_output_id: str):
    """Delete component output."""
    return delete_event(models.ComponentOutput, component_output_id)

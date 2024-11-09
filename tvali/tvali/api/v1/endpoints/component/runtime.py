"""Component runtime endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.component import ComponentRuntime, ComponentRuntimeCreate
from .....db import models

router = APIRouter(
    prefix="/runtime",
    tags=["runtime"],
)


@router.get("/{component_runtime_id}")
async def get_component_runtime(component_runtime_id: str) -> ComponentRuntime:
    """Get component runtime."""
    return get_event(models.ComponentRuntime, ComponentRuntime, component_runtime_id)


@router.post("/")
async def create_component_runtime(event: ComponentRuntimeCreate):
    """Create component runtime."""
    return create_event(event, models.ComponentRuntime)


@router.delete("/{component_runtime_id}")
async def delete_component_runtime(component_runtime_id: str):
    """Delete component runtime."""
    return delete_event(models.ComponentRuntime, component_runtime_id)

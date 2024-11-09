"""Subcomponent event endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.subcomponent import SubcomponentEvent, SubcomponentEventCreate
from .....db import models

router = APIRouter(
    prefix="/event",
    tags=["event"],
)


@router.get("/{subcomponent_event_id}")
async def get_subcomponent_event(subcomponent_event_id: str) -> SubcomponentEvent:
    """Get subcomponent event."""
    return get_event(models.SubcomponentEvent, SubcomponentEvent, subcomponent_event_id)


@router.post("/")
async def create_subcomponent_event(event: SubcomponentEventCreate):
    """Create subcomponent event."""
    return create_event(event, models.SubcomponentEvent)


@router.delete("/{subcomponent_event_id}")
async def delete_subcomponent_event(subcomponent_event_id: str):
    """Delete subcomponent event."""
    return delete_event(models.SubcomponentEvent, subcomponent_event_id)

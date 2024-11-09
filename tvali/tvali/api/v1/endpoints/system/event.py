"""System event endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.system import SystemEvent, SystemEventCreate
from .....db import models

router = APIRouter(
    prefix="/event",
    tags=["event"],
)


@router.get("/{system_event_id}")
async def get_system_event(system_event_id: str) -> SystemEvent:
    """Get system event."""
    return get_event(models.SystemEvent, SystemEvent, system_event_id)


@router.post("/")
async def create_system_event(event: SystemEventCreate):
    """Create system event."""
    return create_event(event, models.SystemEvent)


@router.delete("/{system_event_id}")
async def delete_system_event(system_event_id: str):
    """Delete system event."""
    return delete_event(models.SystemEvent, system_event_id)

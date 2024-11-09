"""Subsystem event endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.subsystem import SubsystemEvent, SubsystemEventCreate
from .....db import models

router = APIRouter(
    prefix="/event",
    tags=["event"],
)


@router.get("/{subsystem_event_id}")
async def get_subsystem_event(subsystem_event_id: str) -> SubsystemEvent:
    """Get subsystem event."""
    return get_event(models.SubsystemEvent, SubsystemEvent, subsystem_event_id)


@router.post("/")
async def create_subsystem_event(event: SubsystemEventCreate):
    """Create subsystem event."""
    return create_event(event, models.SubsystemEvent)


@router.delete("/{subsystem_event_id}")
async def delete_subsystem_event(subsystem_event_id: str):
    """Delete subsystem event."""
    return delete_event(models.SubsystemEvent, subsystem_event_id)

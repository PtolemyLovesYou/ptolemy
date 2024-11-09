"""System output endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.system import SystemOutput, SystemOutputCreate
from .....db import models

router = APIRouter(
    prefix="/output",
    tags=["output"],
)


@router.get("/{system_output_id}")
async def get_system_output(system_output_id: str) -> SystemOutput:
    """Get system output."""
    return get_event(models.SystemOutput, SystemOutput, system_output_id)


@router.post("/")
async def create_system_output(event: SystemOutputCreate):
    """Create system output."""
    return create_event(event, models.SystemOutput)


@router.delete("/{system_output_id}")
async def delete_system_output(system_output_id: str):
    """Delete system output."""
    return delete_event(models.SystemOutput, system_output_id)

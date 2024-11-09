"""Subsystem input endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.subsystem import SubsystemInput, SubsystemInputCreate
from .....db import models

router = APIRouter(
    prefix="/input",
    tags=["input"],
)


@router.get("/{subsystem_input_id}")
async def get_subsystem_input(subsystem_input_id: str) -> SubsystemInput:
    """Get subsystem input."""
    return get_event(models.SubsystemInput, SubsystemInput, subsystem_input_id)


@router.post("/")
async def create_subsystem_input(event: SubsystemInputCreate):
    """Create subsystem input."""
    return create_event(event, models.SubsystemInput)


@router.delete("/{subsystem_input_id}")
async def delete_subsystem_input(subsystem_input_id: str):
    """Delete subsystem input."""
    return delete_event(models.SubsystemInput, subsystem_input_id)

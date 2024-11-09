"""System input endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.system import SystemInput, SystemInputCreate
from .....db import models

router = APIRouter(
    prefix="/input",
    tags=["input"],
)


@router.get("/{system_input_id}")
async def get_system_input(system_input_id: str) -> SystemInput:
    """Get system input."""
    return get_event(models.SystemInput, SystemInput, system_input_id)


@router.post("/")
async def create_system_input(event: SystemInputCreate):
    """Create system input."""
    return create_event(event, models.SystemInput)


@router.delete("/{system_input_id}")
async def delete_system_input(system_input_id: str):
    """Delete system input."""
    return delete_event(models.SystemInput, system_input_id)

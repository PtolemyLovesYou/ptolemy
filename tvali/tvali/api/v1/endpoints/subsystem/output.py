"""Subsystem output endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.subsystem import SubsystemOutput, SubsystemOutputCreate
from .....db import models

router = APIRouter(
    prefix="/output",
    tags=["output"],
)


@router.get("/{subsystem_output_id}")
async def get_subsystem_output(subsystem_output_id: str) -> SubsystemOutput:
    """Get subsystem output."""
    return get_event(models.SubsystemOutput, SubsystemOutput, subsystem_output_id)


@router.post("/")
async def create_subsystem_output(event: SubsystemOutputCreate):
    """Create subsystem output."""
    return create_event(event, models.SubsystemOutput)


@router.delete("/{subsystem_output_id}")
async def delete_subsystem_output(subsystem_output_id: str):
    """Delete subsystem output."""
    return delete_event(models.SubsystemOutput, subsystem_output_id)

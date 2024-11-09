"""Subsystem runtime endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.subsystem import SubsystemRuntime, SubsystemRuntimeCreate
from .....db import models

router = APIRouter(
    prefix="/runtime",
    tags=["runtime"],
)


@router.get("/{subsystem_runtime_id}")
async def get_subsystem_runtime(subsystem_runtime_id: str) -> SubsystemRuntime:
    """Get subsystem runtime."""
    return get_event(models.SubsystemRuntime, SubsystemRuntime, subsystem_runtime_id)


@router.post("/")
async def create_subsystem_runtime(event: SubsystemRuntimeCreate):
    """Create subsystem runtime."""
    return create_event(event, models.SubsystemRuntime)


@router.delete("/{subsystem_runtime_id}")
async def delete_subsystem_runtime(subsystem_runtime_id: str):
    """Delete subsystem runtime."""
    return delete_event(models.SubsystemRuntime, subsystem_runtime_id)

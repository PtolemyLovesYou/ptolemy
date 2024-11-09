"""System runtime endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.system import SystemRuntime, SystemRuntimeCreate
from .....db import models

router = APIRouter(
    prefix="/runtime",
    tags=["runtime"],
)


@router.get("/{system_runtime_id}")
async def get_system_runtime(system_runtime_id: str) -> SystemRuntime:
    """Get system runtime."""
    return get_event(models.SystemRuntime, SystemRuntime, system_runtime_id)


@router.post("/")
async def create_system_runtime(event: SystemRuntimeCreate):
    """Create system runtime."""
    return create_event(event, models.SystemRuntime)


@router.delete("/{system_runtime_id}")
async def delete_system_runtime(system_runtime_id: str):
    """Delete system runtime."""
    return delete_event(models.SystemRuntime, system_runtime_id)

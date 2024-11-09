"""System metadata endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.system import SystemMetadata, SystemMetadataCreate
from .....db import models

router = APIRouter(
    prefix="/metadata",
    tags=["metadata"],
)


@router.get("/{system_metadata_id}")
async def get_system_metadata(system_metadata_id: str) -> SystemMetadata:
    """Get system metadata."""
    return get_event(models.SystemMetadata, SystemMetadata, system_metadata_id)


@router.post("/")
async def create_system_metadata(event: SystemMetadataCreate):
    """Create system metadata."""
    return create_event(event, models.SystemMetadata)


@router.delete("/{system_metadata_id}")
async def delete_system_metadata(system_metadata_id: str):
    """Delete system metadata."""
    return delete_event(models.SystemMetadata, system_metadata_id)

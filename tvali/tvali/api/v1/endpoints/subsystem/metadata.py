"""Subsystem metadata endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.subsystem import SubsystemMetadata, SubsystemMetadataCreate
from .....db import models

router = APIRouter(
    prefix="/metadata",
    tags=["metadata"],
)


@router.get("/{subsystem_metadata_id}")
async def get_subsystem_metadata(subsystem_metadata_id: str) -> SubsystemMetadata:
    """Get subsystem metadata."""
    return get_event(models.SubsystemMetadata, SubsystemMetadata, subsystem_metadata_id)


@router.post("/")
async def create_subsystem_metadata(event: SubsystemMetadataCreate):
    """Create subsystem metadata."""
    return create_event(event, models.SubsystemMetadata)


@router.delete("/{subsystem_metadata_id}")
async def delete_subsystem_metadata(subsystem_metadata_id: str):
    """Delete subsystem metadata."""
    return delete_event(models.SubsystemMetadata, subsystem_metadata_id)

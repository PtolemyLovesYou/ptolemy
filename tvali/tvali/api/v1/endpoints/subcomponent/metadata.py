"""Subcomponent metadata endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.subcomponent import SubcomponentMetadata, SubcomponentMetadataCreate
from .....db import models

router = APIRouter(
    prefix="/metadata",
    tags=["metadata"],
)


@router.get("/{subcomponent_metadata_id}")
async def get_subcomponent_metadata(
    subcomponent_metadata_id: str,
) -> SubcomponentMetadata:
    """Get subcomponent metadata."""
    return get_event(
        models.SubcomponentMetadata, SubcomponentMetadata, subcomponent_metadata_id
    )


@router.post("/")
async def create_subcomponent_metadata(event: SubcomponentMetadataCreate):
    """Create subcomponent metadata."""
    return create_event(event, models.SubcomponentMetadata)


@router.delete("/{subcomponent_metadata_id}")
async def delete_subcomponent_metadata(subcomponent_metadata_id: str):
    """Delete subcomponent metadata."""
    return delete_event(models.SubcomponentMetadata, subcomponent_metadata_id)

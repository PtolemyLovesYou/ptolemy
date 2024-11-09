"""Component metadata endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.component import ComponentMetadata, ComponentMetadataCreate
from .....db import models

router = APIRouter(
    prefix="/metadata",
    tags=["metadata"],
)


@router.get("/{component_metadata_id}")
async def get_component_metadata(component_metadata_id: str) -> ComponentMetadata:
    """Get component metadata."""
    return get_event(models.ComponentMetadata, ComponentMetadata, component_metadata_id)


@router.post("/")
async def create_component_metadata(event: ComponentMetadataCreate):
    """Create component metadata."""
    return create_event(event, models.ComponentMetadata)


@router.delete("/{component_metadata_id}")
async def delete_component_metadata(component_metadata_id: str):
    """Delete component metadata."""
    return delete_event(models.ComponentMetadata, component_metadata_id)

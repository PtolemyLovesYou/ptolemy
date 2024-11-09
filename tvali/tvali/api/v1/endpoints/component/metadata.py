"""Component metadata endpoints."""
from fastapi import APIRouter
from ...schemas.component import ComponentMetadata, ComponentMetadataCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/metadata",
    tags=["metadata"],
)

@router.get("/{component_metadata_id}")
async def get_component_metadata(component_metadata_id: str) -> ComponentMetadata:
    """Get component metadata."""
    db = SessionLocal()
    event = (
        db
        .query(models.ComponentMetadata)
        .filter(models.ComponentEvent.id == component_metadata_id)
        .first()
        )

    return event

@router.post("/")
async def create_component_metadata(event: ComponentMetadataCreate):
    """Create component metadata."""
    db = SessionLocal()
    db_event = models.ComponentMetadata(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {
        "id": db_event.id
    }

@router.delete("/{component_metadata_id}")
async def delete_component_metadata(component_metadata_id: str):
    """Delete component metadata."""
    db = SessionLocal()
    db.query(models.ComponentMetadata).filter(models.ComponentEvent.id == component_metadata_id).delete()
    db.commit()

    return {
        "status": "success"
    }

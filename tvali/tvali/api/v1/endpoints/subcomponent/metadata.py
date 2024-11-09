"""Subcomponent metadata endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event
from ...schemas.subcomponent import SubcomponentMetadata, SubcomponentMetadataCreate
from .....db import models
from .....db.session import SessionLocal

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
    db = SessionLocal()
    db_event = models.SubcomponentMetadata(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{subcomponent_metadata_id}")
async def delete_subcomponent_metadata(subcomponent_metadata_id: str):
    """Delete subcomponent metadata."""
    db = SessionLocal()
    db.query(models.SubcomponentMetadata).filter(
        models.SubcomponentEvent.id == subcomponent_metadata_id
    ).delete()
    db.commit()

    return {"status": "success"}

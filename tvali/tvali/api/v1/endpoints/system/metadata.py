"""System metadata endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event
from ...schemas.system import SystemMetadata, SystemMetadataCreate
from .....db import models
from .....db.session import SessionLocal

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
    db = SessionLocal()
    db_event = models.SystemMetadata(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{system_metadata_id}")
async def delete_system_metadata(system_metadata_id: str):
    """Delete system metadata."""
    db = SessionLocal()
    db.query(models.SystemMetadata).filter(
        models.SystemEvent.id == system_metadata_id
    ).delete()
    db.commit()

    return {"status": "success"}

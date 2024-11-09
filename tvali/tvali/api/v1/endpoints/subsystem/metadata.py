"""Subsystem metadata endpoints."""

from fastapi import APIRouter
from ...schemas.subsystem import SubsystemMetadata, SubsystemMetadataCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/metadata",
    tags=["metadata"],
)


@router.get("/{subsystem_metadata_id}")
async def get_subsystem_metadata(subsystem_metadata_id: str) -> SubsystemMetadata:
    """Get subsystem metadata."""
    db = SessionLocal()
    event = (
        db.query(models.SubsystemMetadata)
        .filter(models.SubsystemEvent.id == subsystem_metadata_id)
        .first()
    )

    return event


@router.post("/")
async def create_subsystem_metadata(event: SubsystemMetadataCreate):
    """Create subsystem metadata."""
    db = SessionLocal()
    db_event = models.SubsystemMetadata(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{subsystem_metadata_id}")
async def delete_subsystem_metadata(subsystem_metadata_id: str):
    """Delete subsystem metadata."""
    db = SessionLocal()
    db.query(models.SubsystemMetadata).filter(
        models.SubsystemEvent.id == subsystem_metadata_id
    ).delete()
    db.commit()

    return {"status": "success"}

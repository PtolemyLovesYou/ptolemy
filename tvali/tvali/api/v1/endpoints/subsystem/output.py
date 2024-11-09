"""Subsystem output endpoints."""
from fastapi import APIRouter
from ...schemas.subsystem import SubsystemOutput, SubsystemOutputCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/output",
    tags=["output"],
)

@router.get("/{subsystem_output_id}")
async def get_subsystem_output(subsystem_output_id: str) -> SubsystemOutput:
    """Get subsystem output."""
    db = SessionLocal()
    event = (
        db
        .query(models.SubsystemOutput)
        .filter(models.SubsystemEvent.id == subsystem_output_id)
        .first()
        )

    return event

@router.post("/")
async def create_subsystem_output(event: SubsystemOutputCreate):
    """Create subsystem output."""
    db = SessionLocal()
    db_event = models.SubsystemOutput(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {
        "id": db_event.id
    }

@router.delete("/{subsystem_output_id}")
async def delete_subsystem_output(subsystem_output_id: str):
    """Delete subsystem output."""
    db = SessionLocal()
    db.query(models.SubsystemOutput).filter(models.SubsystemEvent.id == subsystem_output_id).delete()
    db.commit()

    return {
        "status": "success"
    }

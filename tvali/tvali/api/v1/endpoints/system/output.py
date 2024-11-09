"""System output endpoints."""
from fastapi import APIRouter
from ...schemas.system import SystemOutput, SystemOutputCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/output",
    tags=["output"],
)

@router.get("/{system_output_id}")
async def get_system_output(system_output_id: str) -> SystemOutput:
    """Get system output."""
    db = SessionLocal()
    event = (
        db
        .query(models.SystemOutput)
        .filter(models.SystemEvent.id == system_output_id)
        .first()
        )

    return event

@router.post("/")
async def create_system_output(event: SystemOutputCreate):
    """Create system output."""
    db = SessionLocal()
    db_event = models.SystemOutput(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {
        "id": db_event.id
    }

@router.delete("/{system_output_id}")
async def delete_system_output(system_output_id: str):
    """Delete system output."""
    db = SessionLocal()
    db.query(models.SystemOutput).filter(models.SystemEvent.id == system_output_id).delete()
    db.commit()

    return {
        "status": "success"
    }

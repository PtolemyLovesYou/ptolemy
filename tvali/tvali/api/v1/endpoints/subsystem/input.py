"""Subsystem input endpoints."""

from fastapi import APIRouter
from ...schemas.subsystem import SubsystemInput, SubsystemInputCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/input",
    tags=["input"],
)


@router.get("/{subsystem_input_id}")
async def get_subsystem_input(subsystem_input_id: str) -> SubsystemInput:
    """Get subsystem input."""
    db = SessionLocal()
    event = (
        db.query(models.SubsystemInput)
        .filter(models.SubsystemEvent.id == subsystem_input_id)
        .first()
    )

    return event


@router.post("/")
async def create_subsystem_input(event: SubsystemInputCreate):
    """Create subsystem input."""
    db = SessionLocal()
    db_event = models.SubsystemInput(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{subsystem_input_id}")
async def delete_subsystem_input(subsystem_input_id: str):
    """Delete subsystem input."""
    db = SessionLocal()
    db.query(models.SubsystemInput).filter(
        models.SubsystemEvent.id == subsystem_input_id
    ).delete()
    db.commit()

    return {"status": "success"}

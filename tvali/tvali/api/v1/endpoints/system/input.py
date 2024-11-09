"""System input endpoints."""

from fastapi import APIRouter
from ...schemas.system import SystemInput, SystemInputCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/input",
    tags=["input"],
)


@router.get("/{system_input_id}")
async def get_system_input(system_input_id: str) -> SystemInput:
    """Get system input."""
    db = SessionLocal()
    event = (
        db.query(models.SystemInput)
        .filter(models.SystemEvent.id == system_input_id)
        .first()
    )

    return event


@router.post("/")
async def create_system_input(event: SystemInputCreate):
    """Create system input."""
    db = SessionLocal()
    db_event = models.SystemInput(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{system_input_id}")
async def delete_system_input(system_input_id: str):
    """Delete system input."""
    db = SessionLocal()
    db.query(models.SystemInput).filter(
        models.SystemEvent.id == system_input_id
    ).delete()
    db.commit()

    return {"status": "success"}

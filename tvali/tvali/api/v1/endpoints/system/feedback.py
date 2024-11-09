"""System feedback endpoints."""

from fastapi import APIRouter
from ...schemas.system import SystemFeedback, SystemFeedbackCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/feedback",
    tags=["feedback"],
)


@router.get("/{system_feedback_id}")
async def get_system_feedback(system_feedback_id: str) -> SystemFeedback:
    """Get system feedback."""
    db = SessionLocal()
    event = (
        db.query(models.SystemFeedback)
        .filter(models.SystemEvent.id == system_feedback_id)
        .first()
    )

    return event


@router.post("/")
async def create_system_feedback(event: SystemFeedbackCreate):
    """Create system feedback."""
    db = SessionLocal()
    db_event = models.SystemFeedback(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{system_feedback_id}")
async def delete_system_feedback(system_feedback_id: str):
    """Delete system feedback."""
    db = SessionLocal()
    db.query(models.SystemFeedback).filter(
        models.SystemEvent.id == system_feedback_id
    ).delete()
    db.commit()

    return {"status": "success"}

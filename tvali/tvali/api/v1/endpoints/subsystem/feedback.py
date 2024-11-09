"""Subsystem feedback endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event
from ...schemas.subsystem import SubsystemFeedback, SubsystemFeedbackCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/feedback",
    tags=["feedback"],
)


@router.get("/{subsystem_feedback_id}")
async def get_subsystem_feedback(subsystem_feedback_id: str) -> SubsystemFeedback:
    """Get subsystem feedback."""
    return get_event(models.SubsystemFeedback, SubsystemFeedback, subsystem_feedback_id)


@router.post("/")
async def create_subsystem_feedback(event: SubsystemFeedbackCreate):
    """Create subsystem feedback."""
    db = SessionLocal()
    db_event = models.SubsystemFeedback(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{subsystem_feedback_id}")
async def delete_subsystem_feedback(subsystem_feedback_id: str):
    """Delete subsystem feedback."""
    db = SessionLocal()
    db.query(models.SubsystemFeedback).filter(
        models.SubsystemEvent.id == subsystem_feedback_id
    ).delete()
    db.commit()

    return {"status": "success"}

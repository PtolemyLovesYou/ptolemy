"""Subcomponent feedback endpoints."""

from fastapi import APIRouter
from ...schemas.subcomponent import SubcomponentFeedback, SubcomponentFeedbackCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/feedback",
    tags=["feedback"],
)


@router.get("/{subcomponent_feedback_id}")
async def get_subcomponent_feedback(
    subcomponent_feedback_id: str,
) -> SubcomponentFeedback:
    """Get subcomponent feedback."""
    db = SessionLocal()
    event = (
        db.query(models.SubcomponentFeedback)
        .filter(models.SubcomponentEvent.id == subcomponent_feedback_id)
        .first()
    )

    return event


@router.post("/")
async def create_subcomponent_feedback(event: SubcomponentFeedbackCreate):
    """Create subcomponent feedback."""
    db = SessionLocal()
    db_event = models.SubcomponentFeedback(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{subcomponent_feedback_id}")
async def delete_subcomponent_feedback(subcomponent_feedback_id: str):
    """Delete subcomponent feedback."""
    db = SessionLocal()
    db.query(models.SubcomponentFeedback).filter(
        models.SubcomponentEvent.id == subcomponent_feedback_id
    ).delete()
    db.commit()

    return {"status": "success"}

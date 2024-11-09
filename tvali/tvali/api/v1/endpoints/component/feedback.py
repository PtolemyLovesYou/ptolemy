"""Component feedback endpoints."""

from fastapi import APIRouter
from ...schemas.component import ComponentFeedback, ComponentFeedbackCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/feedback",
    tags=["feedback"],
)


@router.get("/{component_feedback_id}")
async def get_component_feedback(component_feedback_id: str) -> ComponentFeedback:
    """Get component feedback."""
    db = SessionLocal()
    event = (
        db.query(models.ComponentFeedback)
        .filter(models.ComponentEvent.id == component_feedback_id)
        .first()
    )

    return event


@router.post("/")
async def create_component_feedback(event: ComponentFeedbackCreate):
    """Create component feedback."""
    db = SessionLocal()
    db_event = models.ComponentFeedback(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{component_feedback_id}")
async def delete_component_feedback(component_feedback_id: str):
    """Delete component feedback."""
    db = SessionLocal()
    db.query(models.ComponentFeedback).filter(
        models.ComponentEvent.id == component_feedback_id
    ).delete()
    db.commit()

    return {"status": "success"}

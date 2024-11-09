"""Subcomponent feedback endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.subcomponent import SubcomponentFeedback, SubcomponentFeedbackCreate
from .....db import models

router = APIRouter(
    prefix="/feedback",
    tags=["feedback"],
)


@router.get("/{subcomponent_feedback_id}")
async def get_subcomponent_feedback(
    subcomponent_feedback_id: str,
) -> SubcomponentFeedback:
    """Get subcomponent feedback."""
    return get_event(
        models.SubcomponentFeedback, SubcomponentFeedback, subcomponent_feedback_id
    )


@router.post("/")
async def create_subcomponent_feedback(event: SubcomponentFeedbackCreate):
    """Create subcomponent feedback."""
    return create_event(event, models.SubcomponentFeedback)


@router.delete("/{subcomponent_feedback_id}")
async def delete_subcomponent_feedback(subcomponent_feedback_id: str):
    """Delete subcomponent feedback."""
    return delete_event(models.SubcomponentFeedback, subcomponent_feedback_id)

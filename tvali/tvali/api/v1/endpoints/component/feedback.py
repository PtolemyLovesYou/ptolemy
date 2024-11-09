"""Component feedback endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.component import ComponentFeedback, ComponentFeedbackCreate
from .....db import models

router = APIRouter(
    prefix="/feedback",
    tags=["feedback"],
)


@router.get("/{component_feedback_id}")
async def get_component_feedback(component_feedback_id: str) -> ComponentFeedback:
    """Get component feedback."""
    return get_event(models.ComponentFeedback, ComponentFeedback, component_feedback_id)


@router.post("/")
async def create_component_feedback(event: ComponentFeedbackCreate):
    """Create component feedback."""
    return create_event(event, models.ComponentFeedback)


@router.delete("/{component_feedback_id}")
async def delete_component_feedback(component_feedback_id: str):
    """Delete component feedback."""
    return delete_event(models.ComponentFeedback, component_feedback_id)

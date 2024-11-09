"""System feedback endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.system import SystemFeedback, SystemFeedbackCreate
from .....db import models

router = APIRouter(
    prefix="/feedback",
    tags=["feedback"],
)


@router.get("/{system_feedback_id}")
async def get_system_feedback(system_feedback_id: str) -> SystemFeedback:
    """Get system feedback."""
    return get_event(models.SystemFeedback, SystemFeedback, system_feedback_id)


@router.post("/")
async def create_system_feedback(event: SystemFeedbackCreate):
    """Create system feedback."""
    return create_event(event, models.SystemFeedback)


@router.delete("/{system_feedback_id}")
async def delete_system_feedback(system_feedback_id: str):
    """Delete system feedback."""
    return delete_event(models.SystemFeedback, system_feedback_id)

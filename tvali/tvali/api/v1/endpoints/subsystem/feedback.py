"""Subsystem feedback endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.subsystem import SubsystemFeedback, SubsystemFeedbackCreate
from .....db import models

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
    return create_event(event, models.SubsystemFeedback)


@router.delete("/{subsystem_feedback_id}")
async def delete_subsystem_feedback(subsystem_feedback_id: str):
    """Delete subsystem feedback."""
    return delete_event(models.SubsystemFeedback, subsystem_feedback_id)

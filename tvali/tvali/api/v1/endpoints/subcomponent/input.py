"""Subcomponent input endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.subcomponent import SubcomponentInput, SubcomponentInputCreate
from .....db import models

router = APIRouter(
    prefix="/input",
    tags=["input"],
)


@router.get("/{subcomponent_input_id}")
async def get_subcomponent_input(subcomponent_input_id: str) -> SubcomponentInput:
    """Get subcomponent input."""
    return get_event(models.SubcomponentInput, SubcomponentInput, subcomponent_input_id)


@router.post("/")
async def create_subcomponent_input(event: SubcomponentInputCreate):
    """Create subcomponent input."""
    return create_event(event, models.SubcomponentInput)


@router.delete("/{subcomponent_input_id}")
async def delete_subcomponent_input(subcomponent_input_id: str):
    """Delete subcomponent input."""
    return delete_event(models.SubcomponentInput, subcomponent_input_id)

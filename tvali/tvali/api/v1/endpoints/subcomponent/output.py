"""Subcomponent output endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.subcomponent import SubcomponentOutput, SubcomponentOutputCreate
from .....db import models

router = APIRouter(
    prefix="/output",
    tags=["output"],
)


@router.get("/{subcomponent_output_id}")
async def get_subcomponent_output(subcomponent_output_id: str) -> SubcomponentOutput:
    """Get subcomponent output."""
    return get_event(
        models.SubcomponentOutput, SubcomponentOutput, subcomponent_output_id
    )


@router.post("/")
async def create_subcomponent_output(event: SubcomponentOutputCreate):
    """Create subcomponent output."""
    return create_event(event, models.SubcomponentOutput)


@router.delete("/{subcomponent_output_id}")
async def delete_subcomponent_output(subcomponent_output_id: str):
    """Delete subcomponent output."""
    return delete_event(models.SubcomponentOutput, subcomponent_output_id)

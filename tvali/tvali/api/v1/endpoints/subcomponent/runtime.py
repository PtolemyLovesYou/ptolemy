"""Subcomponent runtime endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event, create_event, delete_event
from ...schemas.subcomponent import SubcomponentRuntime, SubcomponentRuntimeCreate
from .....db import models

router = APIRouter(
    prefix="/runtime",
    tags=["runtime"],
)


@router.get("/{subcomponent_runtime_id}")
async def get_subcomponent_runtime(subcomponent_runtime_id: str) -> SubcomponentRuntime:
    """Get subcomponent runtime."""
    return get_event(
        models.SubcomponentRuntime, SubcomponentRuntime, subcomponent_runtime_id
    )


@router.post("/")
async def create_subcomponent_runtime(event: SubcomponentRuntimeCreate):
    """Create subcomponent runtime."""
    return create_event(event, models.SubcomponentRuntime)


@router.delete("/{subcomponent_runtime_id}")
async def delete_subcomponent_runtime(subcomponent_runtime_id: str):
    """Delete subcomponent runtime."""
    return delete_event(models.SubcomponentRuntime, subcomponent_runtime_id)

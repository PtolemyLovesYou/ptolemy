"""Subsystem runtime endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event
from ...schemas.subsystem import SubsystemRuntime, SubsystemRuntimeCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/runtime",
    tags=["runtime"],
)


@router.get("/{subsystem_runtime_id}")
async def get_subsystem_runtime(subsystem_runtime_id: str) -> SubsystemRuntime:
    """Get subsystem runtime."""
    return get_event(models.SubsystemRuntime, SubsystemRuntime, subsystem_runtime_id)


@router.post("/")
async def create_subsystem_runtime(event: SubsystemRuntimeCreate):
    """Create subsystem runtime."""
    db = SessionLocal()
    db_event = models.SubsystemRuntime(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{subsystem_runtime_id}")
async def delete_subsystem_runtime(subsystem_runtime_id: str):
    """Delete subsystem runtime."""
    db = SessionLocal()
    db.query(models.SubsystemRuntime).filter(
        models.SubsystemEvent.id == subsystem_runtime_id
    ).delete()
    db.commit()

    return {"status": "success"}

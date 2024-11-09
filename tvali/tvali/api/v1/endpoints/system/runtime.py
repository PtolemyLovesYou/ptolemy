"""System runtime endpoints."""
from fastapi import APIRouter
from ...schemas.system import SystemRuntime, SystemRuntimeCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/runtime",
    tags=["runtime"],
)

@router.get("/{system_runtime_id}")
async def get_system_runtime(system_runtime_id: str) -> SystemRuntime:
    """Get system runtime."""
    db = SessionLocal()
    event = (
        db
        .query(models.SystemRuntime)
        .filter(models.SystemEvent.id == system_runtime_id)
        .first()
        )

    return event

@router.post("/")
async def create_system_runtime(event: SystemRuntimeCreate):
    """Create system runtime."""
    db = SessionLocal()
    db_event = models.SystemRuntime(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {
        "id": db_event.id
    }

@router.delete("/{system_runtime_id}")
async def delete_system_runtime(system_runtime_id: str):
    """Delete system runtime."""
    db = SessionLocal()
    db.query(models.SystemRuntime).filter(models.SystemEvent.id == system_runtime_id).delete()
    db.commit()

    return {
        "status": "success"
    }

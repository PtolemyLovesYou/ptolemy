"""Subcomponent runtime endpoints."""
from fastapi import APIRouter
from ...schemas.subcomponent import SubcomponentRuntime, SubcomponentRuntimeCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/runtime",
    tags=["runtime"],
)

@router.get("/{subcomponent_runtime_id}")
async def get_subcomponent_runtime(subcomponent_runtime_id: str) -> SubcomponentRuntime:
    """Get subcomponent runtime."""
    db = SessionLocal()
    event = (
        db
        .query(models.SubcomponentRuntime)
        .filter(models.SubcomponentEvent.id == subcomponent_runtime_id)
        .first()
        )

    return event

@router.post("/")
async def create_subcomponent_runtime(event: SubcomponentRuntimeCreate):
    """Create subcomponent runtime."""
    db = SessionLocal()
    db_event = models.SubcomponentRuntime(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {
        "id": db_event.id
    }

@router.delete("/{subcomponent_runtime_id}")
async def delete_subcomponent_runtime(subcomponent_runtime_id: str):
    """Delete subcomponent runtime."""
    db = SessionLocal()
    db.query(models.SubcomponentRuntime).filter(models.SubcomponentEvent.id == subcomponent_runtime_id).delete()
    db.commit()

    return {
        "status": "success"
    }

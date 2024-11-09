"""Component runtime endpoints."""
from fastapi import APIRouter
from ...schemas.component import ComponentRuntime, ComponentRuntimeCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/runtime",
    tags=["runtime"],
)

@router.get("/{component_runtime_id}")
async def get_component_runtime(component_runtime_id: str) -> ComponentRuntime:
    """Get component runtime."""
    db = SessionLocal()
    event = (
        db
        .query(models.ComponentRuntime)
        .filter(models.ComponentEvent.id == component_runtime_id)
        .first()
        )

    return event

@router.post("/")
async def create_component_runtime(event: ComponentRuntimeCreate):
    """Create component runtime."""
    db = SessionLocal()
    db_event = models.ComponentRuntime(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {
        "id": db_event.id
    }

@router.delete("/{component_runtime_id}")
async def delete_component_runtime(component_runtime_id: str):
    """Delete component runtime."""
    db = SessionLocal()
    db.query(models.ComponentRuntime).filter(models.ComponentEvent.id == component_runtime_id).delete()
    db.commit()

    return {
        "status": "success"
    }

"""Subcomponent output endpoints."""
from fastapi import APIRouter
from ...schemas.subcomponent import SubcomponentOutput, SubcomponentOutputCreate
from .....db import models
from .....db.session import SessionLocal

router = APIRouter(
    prefix="/output",
    tags=["output"],
)

@router.get("/{subcomponent_output_id}")
async def get_subcomponent_output(subcomponent_output_id: str) -> SubcomponentOutput:
    """Get subcomponent output."""
    db = SessionLocal()
    event = (
        db
        .query(models.SubcomponentOutput)
        .filter(models.SubcomponentEvent.id == subcomponent_output_id)
        .first()
        )

    return event

@router.post("/")
async def create_subcomponent_output(event: SubcomponentOutputCreate):
    """Create subcomponent output."""
    db = SessionLocal()
    db_event = models.SubcomponentOutput(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {
        "id": db_event.id
    }

@router.delete("/{subcomponent_output_id}")
async def delete_subcomponent_output(subcomponent_output_id: str):
    """Delete subcomponent output."""
    db = SessionLocal()
    db.query(models.SubcomponentOutput).filter(models.SubcomponentEvent.id == subcomponent_output_id).delete()
    db.commit()

    return {
        "status": "success"
    }

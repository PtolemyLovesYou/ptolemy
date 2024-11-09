"""Subcomponent input endpoints."""

from fastapi import APIRouter
from ...crud.event import get_event
from ...schemas.subcomponent import SubcomponentInput, SubcomponentInputCreate
from .....db import models
from .....db.session import SessionLocal

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
    db = SessionLocal()
    db_event = models.SubcomponentInput(**event.model_dump())
    db.add(db_event)
    db.commit()
    db.refresh(db_event)

    return {"id": db_event.id}


@router.delete("/{subcomponent_input_id}")
async def delete_subcomponent_input(subcomponent_input_id: str):
    """Delete subcomponent input."""
    db = SessionLocal()
    db.query(models.SubcomponentInput).filter(
        models.SubcomponentEvent.id == subcomponent_input_id
    ).delete()
    db.commit()

    return {"status": "success"}

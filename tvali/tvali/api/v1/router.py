"""API v1 router."""

from fastapi import APIRouter
from .endpoints.event import router as event_router

router = APIRouter(
    prefix="/v1",
    tags=["v1"],
)

router.include_router(event_router)

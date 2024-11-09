"""System-level endpoints."""
from fastapi import APIRouter
from .event import router as event_router
from .runtime import router as runtime_router

router = APIRouter(
    prefix="/system",
    tags=["system"],
)
router.include_router(event_router)
router.include_router(runtime_router)

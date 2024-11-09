"""Component-level endpoints."""

from fastapi import APIRouter
from .event import router as event_router
from .runtime import router as runtime_router
from .input import router as input_router
from .output import router as output_router
from .feedback import router as feedback_router
from .metadata import router as metadata_router

router = APIRouter(
    prefix="/component",
    tags=["component"],
)
router.include_router(event_router)
router.include_router(runtime_router)
router.include_router(input_router)
router.include_router(output_router)
router.include_router(feedback_router)
router.include_router(metadata_router)

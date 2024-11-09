"""API v1 router."""
from fastapi import APIRouter
from .endpoints.system.router import router as system_router

router = APIRouter(
    prefix="/v1",
    tags=["v1"],
)
router.include_router(system_router)

"""API v1 router."""

from fastapi import APIRouter
from .endpoints.log import router as log_router

router = APIRouter(
    prefix="/v1",
    tags=["v1"],
)

router.include_router(log_router)

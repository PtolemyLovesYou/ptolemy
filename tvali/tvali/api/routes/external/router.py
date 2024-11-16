"""Router."""

from fastapi import APIRouter
from .v1.router import router as v1_router

router = APIRouter(
    prefix="/external",
    tags=["external"],
)

router.include_router(v1_router)

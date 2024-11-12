"""API v1 router."""

from fastapi import APIRouter

router = APIRouter(
    prefix="/v1",
    tags=["v1"],
)

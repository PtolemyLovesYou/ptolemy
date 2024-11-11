"""API v1 router."""

from fastapi import APIRouter
from .endpoints.event import router as event_router
from .graphql.router import router as graphql_router

router = APIRouter(
    prefix="/v1",
    tags=["v1"],
)

router.include_router(event_router)
router.include_router(graphql_router, prefix="/graphql", tags=["graphql"])

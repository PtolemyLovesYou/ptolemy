"""Routes."""

from fastapi import APIRouter
from .routes.query import router as query_router


def get_routes() -> APIRouter:
    """Get api router."""
    router = APIRouter()
    router.add_api_route("/query", query_router)

    return router

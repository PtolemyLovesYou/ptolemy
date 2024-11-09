"""API v1 router."""

from fastapi import APIRouter
from .endpoints.system.router import router as system_router
from .endpoints.subsystem.router import router as subsystem_router
from .endpoints.component.router import router as component_router
from .endpoints.subcomponent.router import router as subcomponent_router

router = APIRouter(
    prefix="/v1",
    tags=["v1"],
)
router.include_router(system_router)
router.include_router(subsystem_router)
router.include_router(component_router)
router.include_router(subcomponent_router)

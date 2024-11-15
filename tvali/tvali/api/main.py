"""Tvali API"""

from fastapi import FastAPI
from .core.lifespan import lifespan
from .routes.health import router as health_router
from .routes.v1.router import router as v1_router

# import all models so they actually get created
from .db.models import *  # pylint: disable=unused-wildcard-import,wildcard-import

app = FastAPI(
    title="Tvali API",
    description="Tvali API",
    version="0.0.1",
    docs_url="/swagger",
    lifespan=lifespan,
)

app.include_router(health_router)
app.include_router(v1_router)

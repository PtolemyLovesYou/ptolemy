"""Tvali API"""

from fastapi import FastAPI
from .api.health import router as health_router
from .api.v1.router import router as v1_router
from .db.session import Base, engine

# import all models so they actually get created
from .db.models import *  # pylint: disable=unused-wildcard-import,wildcard-import

Base.metadata.create_all(bind=engine)

app = FastAPI(
    title="Tvali API",
    description="Tvali API",
    version="0.0.1",
    docs_url="/swagger",
)

app.include_router(health_router)
app.include_router(v1_router)

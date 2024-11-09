"""Tvali API"""

from fastapi import FastAPI
from .api.health import router as health_router
from .api.v1.router import router as v1_router
from .db.session import Base, engine
from .db.models import *

Base.metadata.create_all(bind=engine)

app = FastAPI()
app.include_router(health_router)
app.include_router(v1_router)

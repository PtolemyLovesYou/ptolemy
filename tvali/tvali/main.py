"""Tvali API"""
from fastapi import FastAPI
from .api.health import router as health_router
from .db.session import Base, engine
from .db.models import *

Base.metadata.create_all(bind=engine)

app = FastAPI()
app.include_router(health_router)

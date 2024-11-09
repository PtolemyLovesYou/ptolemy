"""Tvali API"""
from fastapi import FastAPI
from .api.health import router as health_router

app = FastAPI()
app.include_router(health_router)

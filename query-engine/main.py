"""Query engine."""

from fastapi import FastAPI
from query_engine.routes import get_routes

app = FastAPI()

app.include_router(get_routes())

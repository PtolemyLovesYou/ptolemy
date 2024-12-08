"""Ptolemy API"""

from fastapi import FastAPI
from .core.lifespan import lifespan
from .routes.health import router as health_router
from .routes.graphql.router import router as graphql_router

app = FastAPI(
    title="Ptolemy API",
    description="Ptolemy API",
    version="0.0.1",
    docs_url="/swagger",
    lifespan=lifespan,
)

app.include_router(health_router)
app.include_router(graphql_router)

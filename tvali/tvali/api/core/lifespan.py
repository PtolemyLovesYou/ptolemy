"""Lifespan function."""

import logging
from contextlib import asynccontextmanager
from fastapi import FastAPI
from ...db.session import engine, Base

logger = logging.getLogger(__name__)


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Lifespan function."""
    logger.info("Starting service %s", app.title)
    logger.info("Creating db tables...")
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)

    yield

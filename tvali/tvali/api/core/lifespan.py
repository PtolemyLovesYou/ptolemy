"""Lifespan function."""

import logging
import asyncio
from contextlib import asynccontextmanager
from fastapi import FastAPI
from redis.asyncio import Redis
from ...db.session import engine, Base
from ...publisher.main import listen

logger = logging.getLogger(__name__)


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Lifespan function."""
    logger.info("Starting service %s", app.title)
    logger.info("Creating db tables...")
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)

    asyncio.create_task(listen(Redis(host="redis", port=6379, db=0), "tvali_stream"))

    yield

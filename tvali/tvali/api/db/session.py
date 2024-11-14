"""Session dependencies"""

from typing import AsyncGenerator
from contextlib import asynccontextmanager
from sqlalchemy.ext.asyncio import create_async_engine, async_sessionmaker, AsyncSession
from sqlalchemy.orm import declarative_base
from ..config import (
    POSTGRES_DB,
    POSTGRES_USER,
    POSTGRES_PASSWORD,
    POSTGRES_HOST,
    POSTGRES_PORT,
)

# Configuration
SQLALCHEMY_DATABASE_URL = "".join(
    [
        "postgresql+psycopg://",
        POSTGRES_USER,
        ":",
        POSTGRES_PASSWORD,
        "@",
        POSTGRES_HOST,
        ":",
        str(POSTGRES_PORT),
        "/",
        POSTGRES_DB,
    ]
)

# Create database engine
engine = create_async_engine(
    SQLALCHEMY_DATABASE_URL,
    # For SQLite, add: connect_args={"check_same_thread": False}
    pool_pre_ping=True,  # Enable automatic reconnection
    pool_size=5,  # Connection pool size
    max_overflow=10,  # Max number of connections beyond pool_size
)

# Create session factory
SessionLocal = async_sessionmaker(
    bind=engine, autocommit=False, autoflush=False, expire_on_commit=False
)

# Base class for declarative models
Base = declarative_base()


@asynccontextmanager
async def get_db() -> AsyncGenerator[AsyncSession, None]:
    """
    FastAPI dependency that provides a database session.
    Usage:
        @app.get("/users/")
        async def get_users(db: AsyncSession = Depends(get_db)) -> None:
            ...
    """
    db = SessionLocal()
    try:
        yield db
    finally:
        await db.close()

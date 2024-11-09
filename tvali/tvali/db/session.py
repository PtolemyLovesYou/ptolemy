"""Session dependencies"""
from typing import Generator
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, Session
from sqlalchemy.ext.declarative import declarative_base
from ..config import (
    POSTGRES_DB,
    POSTGRES_USER,
    POSTGRES_PASSWORD,
    POSTGRES_HOST,
    POSTGRES_PORT
)

# Configuration
SQLALCHEMY_DATABASE_URL = ''.join(
    [
        'postgresql+psycopg://',
        POSTGRES_USER,
        ':',
        POSTGRES_PASSWORD,
        '@',
        POSTGRES_HOST,
        ':',
        str(POSTGRES_PORT),
        '/',
        POSTGRES_DB
    ]
)

# Create database engine
engine = create_engine(
    SQLALCHEMY_DATABASE_URL,
    # For SQLite, add: connect_args={"check_same_thread": False}
    pool_pre_ping=True,  # Enable automatic reconnection
    pool_size=5,         # Connection pool size
    max_overflow=10      # Max number of connections beyond pool_size
)

# Create session factory
SessionLocal = sessionmaker(
    bind=engine,
    autocommit=False,
    autoflush=False,
    expire_on_commit=False
)

# Base class for declarative models
Base = declarative_base()

def get_db() -> Generator[Session, None, None]:
    """
    FastAPI dependency that provides a database session.
    Usage:
        @app.get("/users/")
        def get_users(db: Session = Depends(get_db)):
            ...
    """
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

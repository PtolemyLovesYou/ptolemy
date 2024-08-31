"""Database utilities."""
from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker
from .env_settings import POSTGRES_DB, POSTGRES_USER, POSTGRES_PASSWORD

SQLALCHEMY_DATABASE_URL = "sqlite:///./api/tvali.db"
# SQLALCHEMY_DATABASE_URL = f"postgresql+psycopg://{POSTGRES_USER}:{POSTGRES_PASSWORD}@localhost:5432/{POSTGRES_DB}"

engine = create_engine(SQLALCHEMY_DATABASE_URL, connect_args={"check_same_thread": False})

SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()

def get_db():
    """Get database."""
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

"""Main."""

from fastapi import FastAPI
from .db import engine, Base

Base.metadata.create_all(bind=engine)

app = FastAPI()


@app.get("/")
async def index():
    """
    Root endpoint.

    Returns a simple message.
    """
    return {"message": "Hello World"}


@app.get("/health")
async def health():
    """
    Health endpoint.

    Returns a simple message indicating that the service is healthy.
    """
    return {"status": "ok"}

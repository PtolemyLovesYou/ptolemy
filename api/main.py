from fastapi import FastAPI
from .db import engine, Base

Base.metadata.create_all(bind=engine)

app = FastAPI()

@app.get("/")
async def index():
    return {"message": "Hello World"}


@app.get("/health")
async def health():
    return {"status": "ok"}


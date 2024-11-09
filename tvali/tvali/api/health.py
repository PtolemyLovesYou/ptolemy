"""Health endpoint."""
from fastapi import APIRouter

router = APIRouter(
    prefix="/health",
    tags=["health"],
)

@router.get("/")
def health():
    """Health endpoint."""
    return {"status": "ok"}

"""Publish routes."""

from fastapi import APIRouter

router = APIRouter(
    prefix="/publish",
    tags=["publish"],
)

"""Tvali Config."""

from ..config import TransportConfig


class TvaliConfig(TransportConfig):
    """Tvali Config."""

    base_url: str = "http://localhost:8000"

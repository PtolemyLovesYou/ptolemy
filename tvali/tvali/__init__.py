"""Tvali."""
from importlib.metadata import distribution

_INSTALLED_EXTRAS = distribution(__name__).metadata.json.get("provides_extra") or []

if "client" in _INSTALLED_EXTRAS:
    from .client import *

if "api" in _INSTALLED_EXTRAS:
    from .api import *

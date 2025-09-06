"""Header file for ptolemy core."""

# pylint: disable=unused-argument,missing-function-docstring,too-few-public-methods
from __future__ import annotations
from typing import Any
from uuid import UUID

class RecordExporter:
    """Record Exporter."""

    def __init__(self, base_url: str): ...
    def send_trace(self, trace: Any): ...

def validate_field_value(val: Any, max_size: int = 1024) -> None: ...

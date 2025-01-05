"""Header file for ptolemy core."""

# pylint: disable=unused-argument,missing-function-docstring
from __future__ import annotations
from typing import Optional, Any, Dict

class Ptolemy:
    """Ptolemy Client."""

    def __init__(
        self, workspace_name: str, autoflush: bool, batch_size: int
    ) -> "Ptolemy": ...
    def trace(
        self,
        name: str,
        parameters: Optional[Dict[str, Any]] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> "Ptolemy": ...
    def child(
        self,
        name: str,
        parameters: Optional[Dict[str, Any]] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> "Ptolemy": ...
    def event(
        self,
        name: str,
        parameters: Optional[Dict[str, Any]] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> None: ...
    def runtime(
        self,
        start_time: float,
        end_time: float,
        error_type: Optional[str] = None,
        error_content: Optional[str] = None,
    ) -> None: ...
    def inputs(self, **kwds: Any) -> None: ...
    def outputs(self, **kwds: Any) -> None: ...
    def feedback(self, **kwds: Any) -> None: ...
    def metadata(self, **kwds: Any) -> None: ...
    def push_event(self) -> bool: ...
    def push_io(self) -> bool: ...
    def flush(self) -> bool: ...

"""Header file for ptolemy core."""
from __future__ import annotations
from typing import List, Optional, Any

class BlockingObserverClient:
    """Blocking Observer Client."""
    def __init__(self, batch_size: int) -> None: ...
    def queue(self, records: List[ProtoRecord]) -> bool: ...
    def queue_event(self, record: ProtoRecord) -> bool: ...
    def queue_size(self) -> int: ...
    def flush(self) -> bool: ...

class ProtoRecord:
    """Proto Record."""

class ProtoRecordHandler:
    """Handler for ProtoRecord."""
    @staticmethod
    def event(
        tier: str,
        parent_id: str,
        name: str,
        id: Optional[str] = None,
        parameters: Optional[dict] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> None: ...
    @staticmethod
    def runtime(
        tier: str,
        parent_id: str,
        start_time: float,
        end_time: float,
        id: Optional[str] = None,
        error_type: Optional[str] = None,
        error_content: Optional[str] = None,
    ) -> None: ...
    @staticmethod
    def io(
        tier: str,
        log_type: str,
        parent_id: str,
        field_name: str,
        field_value: Any,
        id: Optional[str] = None,
    ) -> None: ...
    @staticmethod
    def metadata(
        tier: str,
        parent_id: str,
        field_name: str,
        field_value: str,
        id: Optional[str] = None,
    ) -> None: ...

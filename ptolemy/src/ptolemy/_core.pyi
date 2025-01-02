"""Header file for ptolemy core."""
from __future__ import annotations
from typing import List, Optional, Any, Dict

class BlockingObserverClient:
    """Blocking Observer Client."""
    def __init__(self, batch_size: int) -> None: ...
    def queue_event_record(
        self,
        tier: str,
        parent_id: str,
        name: str,
        parameters: Optional[dict] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> str: ...
    def queue_runtime_record(
        self,
        tier: str,
        parent_id: str,
        start_time: float,
        end_time: float,
        error_type: Optional[str] = None,
        error_content: Optional[str] = None,
    ) -> str: ...
    def queue_input_records(
        self,
        tier: str,
        parent_id: str,
        data: Dict[str, Any]
    ) -> None: ...
    def queue_output_records(
        self,
        tier: str,
        parent_id: str,
        data: Dict[str, Any]
    ) -> None: ...
    def queue_feedback_records(
        self,
        tier: str,
        parent_id: str,
        data: Dict[str, Any]
    ) -> None: ...
    def queue_metadata_records(
        self,
        tier: str,
        parent_id: str,
        data: Dict[str, str]
    ) -> None: ...
    def queue(self, records: List[ProtoRecord]) -> bool: ...
    def queue_event(self, record: ProtoRecord) -> bool: ...
    def queue_size(self) -> int: ...
    def flush(self) -> bool: ...

class ProtoRecord:
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
    ) -> 'ProtoRecord': ...
    @staticmethod
    def runtime(
        tier: str,
        parent_id: str,
        start_time: float,
        end_time: float,
        id: Optional[str] = None,
        error_type: Optional[str] = None,
        error_content: Optional[str] = None,
    ) -> 'ProtoRecord': ...
    @staticmethod
    def io(
        tier: str,
        log_type: str,
        parent_id: str,
        field_name: str,
        field_value: Any,
        id: Optional[str] = None,
    ) -> 'ProtoRecord': ...
    @staticmethod
    def metadata(
        tier: str,
        parent_id: str,
        field_name: str,
        field_value: str,
        id: Optional[str] = None,
    ) -> 'ProtoRecord': ...

from __future__ import annotations
from typing import List

class ProtoRecord: ...

class RecordBuilder:
    def __init__(self) -> None: ...
    @staticmethod
    def event(
        tier: str,
        parent_id: str,
        id: str,
        name: str,
        parameters: str | None,
        version: str | None,
        environment: str | None,
    ) -> ProtoRecord: ...
    @staticmethod
    def runtime(
        tier: str,
        parent_id: str,
        start_time: float,
        end_time: float,
        error_type: str | None,
        error_content: str | None,
    ) -> ProtoRecord: ...
    @staticmethod
    def input(
        tier: str, parent_id: str, id: str, field_name: str, field_value: str
    ) -> ProtoRecord: ...
    @staticmethod
    def output(
        tier: str, parent_id: str, id: str, field_name: str, field_value: str
    ) -> ProtoRecord: ...
    @staticmethod
    def feedback(
        tier: str, parent_id: str, id: str, field_name: str, field_value: str
    ) -> ProtoRecord: ...
    @staticmethod
    def metadata(
        tier: str, parent_id: str, id: str, field_name: str, field_value: str
    ) -> ProtoRecord: ...

class BlockingObserverClient:
    def __init__(self, batch_size: int) -> None: ...
    def queue(self, records: List[ProtoRecord]) -> bool: ...
    def queue_size(self) -> int: ...
    def flush(self) -> bool: ...

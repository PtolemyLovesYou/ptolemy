"""Status."""

from enum import StrEnum

class QueryStatus(StrEnum):
    """Query Status."""

    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"

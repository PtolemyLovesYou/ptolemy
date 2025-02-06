"""Enums."""

from enum import StrEnum


class JobStatus(StrEnum):
    """Job status."""

    PENDING = "pending"
    ACCEPTED = "accepted"
    RUNNING = "runnning"
    DONE = "done"
    FAILED = "failed"
    CANCELLED = "cancelled"

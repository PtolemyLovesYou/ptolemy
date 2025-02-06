"""Database."""

from typing import Optional
from pydantic import BaseModel, ConfigDict, RootModel
from redis import Redis
from .models.enums import JobStatus

class QueryBatch(BaseModel):
    """Query batch."""
    data: Optional[bytes] = None
    error: Optional[str] = None
    success: bool

class RedisConn(RootModel):
    """Redis connection."""
    model_config = ConfigDict(arbitrary_types_allowed=True)

    root: Redis

    def create_job_status(self, query_id: str):
        """Create job status."""
        self.root.set(f"ptolemy:status:{query_id}", JobStatus.PENDING, ex=300, keepttl=True)

    def set_job_status(self, query_id: str, status: JobStatus):
        """Set job status."""
        self.root.set(f"ptolemy:status:{query_id}", status.value, keepttl=True)

    def get_job_status(self, query_id: str) -> JobStatus:
        """Get job status."""
        status = self.root.get(f"ptolemy:status:{query_id}")

        return JobStatus(status)

    def push_batch(self, query_id: str, batch: QueryBatch):
        """Push batch."""
        self.root.xadd(f"ptolemy:query:{query_id}", batch)

    def remove_job_status(self, query_id: str):
        """Cancel job."""
        self.root.delete(f"ptolemy:status:{query_id}")

"""Query models."""

from pydantic import BaseModel
from .enums import JobStatus

class CreateQueryRequest(BaseModel):
    """Create query request."""
    schema_name: str
    role_name: str
    query_content: str


class CreateQueryResponse(BaseModel):
    """Create query response."""
    query_id: str
    status: JobStatus

class GetQueryResponse(BaseModel):
    """Get query response."""
    query_id: str
    status: str

"""Query routes."""

import uuid
import duckdb
from fastapi import APIRouter, Depends, HTTPException
from ..models.query import (
    CreateQueryRequest,
    CreateQueryResponse,
    GetQueryResponse
    )
from ..query_executor import QueryExecutor
from ..state import AppState, get_state
from ..models.enums import JobStatus

router = APIRouter()

@router.post("/new")
def new_query(
    data: CreateQueryRequest,
    state: AppState = Depends(get_state)
    ) -> CreateQueryResponse:
    """Create new query."""
    redis_conn = state.get_redis_conn()
    executor = QueryExecutor(
        conn=duckdb.connect(":memory:"),
        redis_conn=redis_conn,
        query_id=str(uuid.uuid4()),
        schema_name=data.schema_name,
        role_name=data.role_name,
        query=data.query_content,
    )

    redis_conn.create_job_status(executor.query_id)

    state.submit_job(executor)

    return CreateQueryResponse(
        query_id=executor.query_id
    )

@router.get("/{query_id}")
def get_query(query_id: str, state: AppState = Depends(get_state)) -> GetQueryResponse:
    """Get query information."""
    conn = state.get_redis_conn()
    status = conn.get_job_status(query_id)

    return GetQueryResponse(
        query_id=query_id,
        status=status
    )

@router.delete("/{query_id}")
def cancel_job(query_id: str, state: AppState = Depends(get_state)):
    """Delete/cancel query."""
    job_cancelled = state.cancel_job(query_id)

    if job_cancelled:
        conn = state.get_redis_conn()
        conn.set_job_status(query_id, JobStatus.CANCELLED)
    else:
        raise HTTPException(status_code=404, detail="Query not found")

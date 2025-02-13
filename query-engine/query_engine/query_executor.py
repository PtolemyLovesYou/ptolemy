"""Query Executor."""
import logging
import os
import json
from enum import StrEnum
from typing import Optional, List
from io import BytesIO
from functools import cached_property
import redis
import duckdb
from pydantic import BaseModel, ConfigDict
from .env_settings import REDIS_DB, REDIS_HOST, REDIS_PORT

class QueryStatus(StrEnum):
    """Query Status."""
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"

LOAD_EXT = """
install postgres;
load postgres;
"""

ATTACH_DB = """
attach database '{}' as ptolemy (type postgres, schema 'duckdb', read_only);
"""

SET_WORKSPACE = """
call postgres_execute('ptolemy', 'set ptolemy.current_workspaces = ''{}''');
"""

SET_ROLE = """
call postgres_execute('ptolemy', 'set role = ptolemy_duckdb');
"""

class QueryExecutor(BaseModel):
    """Query Executor."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    logger: logging.Logger
    query_id: str
    allowed_workspace_ids: List[str]
    query: str
    conn: Optional[duckdb.DuckDBPyConnection] = None

    @cached_property
    def redis_conn(self) -> redis.Redis:
        """Redis connection."""
        return redis.Redis(host=REDIS_HOST, port=REDIS_PORT, db=REDIS_DB)

    @cached_property
    def keyspace(self) -> str:
        """Keyspace."""
        return f"ptolemy:query:{self.query_id}"

    @property
    def database_url(self) -> str:
        """Database url."""
        user = os.getenv("POSTGRES_USER", "postgres")
        password = os.getenv("POSTGRES_PASSWORD", "postgres")
        host = os.getenv("POSTGRES_HOST", "localhost")
        db = os.getenv("POSTGRES_DB", "ptolemy")
        postgres_port = os.getenv("POSTGRES_PORT", "5432")
        return f"postgresql://{user}:{password}@{host}:{postgres_port}/{db}"

    def setup_conn(self):
        """Set up connection."""
        if self.conn is None:
            self.logger.info("Creating new connection")
            self.conn = duckdb.connect(":memory:")

        self.logger.info("Loading extension")
        self.conn.execute(LOAD_EXT)
        self.logger.info("Setting up connection")
        self.conn.execute(ATTACH_DB.format(self.database_url))
        self.logger.info("Setting workspace and role")
        self.conn.execute(SET_WORKSPACE.format(','.join(self.allowed_workspace_ids)))
        self.conn.execute(SET_ROLE)
        self.conn.commit()

    def __call__(self) -> bool:
        if self.redis_conn.hexists(self.keyspace, "status"):
            self.logger.error("Query %s already executed. Overwriting.", self.query_id)

        self.logger.info("Executing query %s", self.query_id)
        self.logger.info("Setting status to running")
        self.redis_conn.hset(
            self.keyspace,
            "status",
            QueryStatus.RUNNING
        )

        self.setup_conn()

        try:
            self.logger.debug("Executing query %s", self.query_id)
            results = self.conn.sql(self.query).arrow().to_pandas()

        except Exception as e:  # pylint: disable=broad-except
            self.logger.info("Error executing query %s", self.query_id, exc_info=e)
            self.redis_conn.hset(
                self.keyspace,
                mapping={
                    "status": QueryStatus.FAILED,
                    "error": str(e)
                }
            )
            # Optionally set expiry time (e.g., 1 hour)
            self.redis_conn.expire(self.keyspace, 3600)
            return 1

        total_rows = results.shape[0]
        est_size = results.memory_usage(deep=True).sum()

        column_names = results.columns.tolist()
        column_types = results.dtypes.tolist()
        total_batches = 0

        for i in range(0, len(results), 100):
            total_batches += 1
            buf = BytesIO()
            dff = results[i:i+100]
            dff.to_feather(buf)

            self.logger.debug("Storing result batch %d for query %s", i, self.query_id)
            self.redis_conn.hset(self.keyspace, f"result:{i}", buf.getvalue())

        self.logger.debug(
            "Query %s executed with %d result batches of size %.2f kB",
            self.query_id,
            total_batches,
            est_size / 1024
            )

        query_metadata = {
            "status": QueryStatus.COMPLETED,
            "metadata:total_rows": total_rows,
            "metadata:total_batches": total_batches,
            "metadata:est_size_bytes": int(est_size),
            "metadata:column_names": json.dumps(column_names),
            "metadata:column_types": json.dumps([str(i) for i in column_types])
        }

        # Store the total number of result batches
        self.redis_conn.hset(
            self.keyspace,
            mapping=query_metadata
            )

        # Set expiry time (e.g., 1 hour)
        self.redis_conn.expire(self.keyspace, 3600)

        return 0

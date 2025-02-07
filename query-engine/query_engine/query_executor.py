"""Query Executor."""
import logging
from typing import Optional
from io import BytesIO
from functools import cached_property
import redis
import duckdb
from pydantic import BaseModel, ConfigDict

class QueryExecutor(BaseModel):
    """Query Executor."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    logger: logging.Logger
    query_id: str
    schema_name: str
    role_name: str
    query: str
    conn: Optional[duckdb.DuckDBPyConnection] = None

    @cached_property
    def redis_conn(self) -> redis.Redis:
        """Redis connection."""
        return redis.Redis(host="localhost", port=6379, db=0)

    @cached_property
    def keyspace(self) -> str:
        """Keyspace."""
        return f"ptolemy:query:{self.query_id}"

    def setup_conn(self):
        """Set up connection."""
        if self.conn is None:
            self.conn = duckdb.connect(":memory:")

    def __call__(self) -> bool:
        if self.redis_conn.hexists(self.keyspace, "status"):
            self.logger.error("Query %s already executed. Overwriting.", self.query_id)

        self.redis_conn.hset(
            self.keyspace,
            "status",
            "running"
        )

        self.setup_conn()

        try:
            self.logger.debug("Executing query %s", self.query_id)
            results = self.conn.sql(self.query).df()

        except Exception as e:  # pylint: disable=broad-except
            self.logger.info("Error executing query %s", self.query_id, exc_info=e)
            self.redis_conn.hset(
                self.keyspace,
                mapping={
                    "status": "error",
                    "error": str(e)
                }
            )
            # Optionally set expiry time (e.g., 1 hour)
            self.redis_conn.expire(self.keyspace, 3600)
            return 1

        results_serialized = []

        for i in range(0, len(results), 100):
            buf = BytesIO()
            dff = results[i:i+100]
            dff.to_feather(buf)
            results_serialized.append(buf.getvalue())

        # Initialize the hash with success status
        self.redis_conn.hset(self.keyspace, "status", "success")

        # Store each batch as a field in the hash
        for idx, result in enumerate(results_serialized):
            self.logger.debug("Storing result batch %d for query %s", idx, self.query_id)
            self.redis_conn.hset(self.keyspace, f"result:{idx}", result)

        # Store the total number of result batches
        self.redis_conn.hset(self.keyspace, "total_batches", len(results_serialized))

        # Set expiry time (e.g., 1 hour)
        self.redis_conn.expire(self.keyspace, 3600)

        return 0

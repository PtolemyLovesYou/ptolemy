"""Query Executor."""

import duckdb
from pydantic import BaseModel, ConfigDict
from .db import RedisConn, QueryBatch
from .models.enums import JobStatus


class QueryExecutor(BaseModel):
    """Query Executor."""

    model_config = ConfigDict(arbitrary_types_allowed=True)

    conn: duckdb.DuckDBPyConnection
    redis_conn: RedisConn

    query_id: str
    schema_name: str
    role_name: str

    query: str

    def setup(self):
        """Set up."""
        self.redis_conn.root.xgroup_create(
            f"ptolemy:query:{self.query_id}",
            f"ptolemy:query:{self.query_id}",
            mkstream=True,
        )

        self.redis_conn.set_job_status(self.query_id, JobStatus.RUNNING)

    def teardown(self):
        """Teardown."""
        self.conn.close()

    def execute(self) -> bool:
        """Execute query."""
        self.redis_conn.set_job_status(self.query_id, JobStatus.ACCEPTED)

        self.setup()

        try:
            results = self.conn.sql(self.query).fetch_arrow_reader(batch_size=100)
            for batch in results:
                self.redis_conn.push_batch(
                    QueryBatch(data=batch.serialize().to_pybytes(), success=True)
                )
            return 0
        except duckdb.Error as e:
            self.teardown()
            self.redis_conn.push_batch(QueryBatch(error=str(e), success=False))
            return 1
        finally:
            self.redis_conn.set_job_status(self.query_id, JobStatus.DONE)
            self.teardown()

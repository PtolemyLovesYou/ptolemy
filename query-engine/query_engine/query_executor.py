"""Query Executor."""

import logging
import os
import json
from typing import Optional, List
from io import BytesIO
from functools import cached_property
import redis
import duckdb
from pydantic import BaseModel, ConfigDict
import pandas as pd
from .status import QueryStatus
from .env_settings import REDIS_DB, REDIS_HOST, REDIS_PORT, STREAM_NAME

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
    batch_size: int = 100
    query: str
    conn: Optional[duckdb.DuckDBPyConnection] = None

    @cached_property
    def redis_conn(self) -> redis.Redis:
        """Redis connection."""
        return redis.Redis(host=REDIS_HOST, port=REDIS_PORT, db=REDIS_DB)

    @cached_property
    def keyspace(self) -> str:
        """Keyspace."""
        return f"{STREAM_NAME}:{self.query_id}"

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

        self.logger.info("Configuring")
        self.conn.execute(LOAD_EXT)
        self.conn.execute(ATTACH_DB.format(self.database_url))
        self.conn.execute(SET_WORKSPACE.format(",".join(self.allowed_workspace_ids)))
        self.conn.execute(SET_ROLE)
        self.conn.commit()

    def init(self) -> None:
        """Initialize."""
        if self.redis_conn.hexists(self.keyspace, "status"):
            self.logger.error("Query %s already executed. Overwriting.", self.query_id)

        self.logger.info("Initializing query %s", self.query_id)
        self.redis_conn.hset(
            self.keyspace,
            mapping={"status": QueryStatus.PENDING},
        )
        self.redis_conn.expire(self.keyspace, 3600)

    def __call__(self) -> bool:
        self.logger.info("Executing query %s", self.query_id)
        self.logger.info("Setting status to running")
        self.redis_conn.hset(self.keyspace, "status", QueryStatus.RUNNING)

        try:
            self.setup_conn()
            self.logger.debug("Executing query %s", self.query_id)
            results: pd.DataFrame = self.conn.sql(self.query).arrow().to_pandas()

        except Exception as e:  # pylint: disable=broad-except
            self.logger.info("Error executing query %s", self.query_id, exc_info=e)
            self.redis_conn.hset(
                self.keyspace, mapping={"status": QueryStatus.FAILED, "error": str(e)}
            )
            return 1
        finally:
            self.logger.debug("Closing connection")
            self.conn.close()

        total_rows = results.shape[0]
        est_size = results.memory_usage(deep=True).sum()

        column_names = results.columns.tolist()
        column_types = results.dtypes.tolist()
        total_batches = 0

        batches = {}

        for batch_id, i in enumerate(range(0, len(results), self.batch_size)):
            buf = BytesIO()
            dff = results[i : i + self.batch_size]
            if len(dff) > 0:
                total_batches += 1
                dff.to_feather(buf)

                self.logger.debug(
                    "Storing result batch %d for query %s", batch_id, self.query_id
                )
                batches[f"result:{batch_id}"] = buf.getvalue()

        if batches:
            self.redis_conn.hset(self.keyspace, mapping=batches)

        self.logger.info(
            "Query %s executed with %d result batches of size %.2f kB",
            self.query_id,
            total_batches,
            est_size / 1024,
        )

        query_metadata = {
            "status": QueryStatus.COMPLETED,
            "metadata:total_rows": total_rows,
            "metadata:total_batches": total_batches,
            "metadata:est_size_bytes": int(est_size),
            "metadata:column_names": json.dumps(column_names),
            "metadata:column_types": json.dumps([str(i) for i in column_types]),
        }

        # Store the total number of result batches
        self.redis_conn.hset(self.keyspace, mapping=query_metadata)

        return 0

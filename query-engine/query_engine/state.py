"""State."""

import os
from typing import Dict
from concurrent.futures import ThreadPoolExecutor, Future
from pydantic import BaseModel, ConfigDict
from redis import BlockingConnectionPool, Redis
from .query_executor import QueryExecutor
from .db import RedisConn

class AppState(BaseModel):
    """App state."""
    model_config = ConfigDict(arbitrary_types_allowed=True)

    redis_pool: BlockingConnectionPool
    executor: ThreadPoolExecutor
    jobs: Dict[str, Future]

    @classmethod
    def new(cls) -> 'AppState':
        """Create new state."""
        redis_host = os.getenv('REDIS_HOST', 'localhost')
        redis_port = os.getenv('REDIS_PORT', "6379")
        redis_db = os.getenv('REDIS_DB', "0")

        redis_pool = BlockingConnectionPool.from_url(
            f"redis://{redis_host}:{redis_port}/{redis_db}"
            )

        executor = ThreadPoolExecutor(max_workers=10)
        jobs = {}

        return cls(redis_pool=redis_pool, executor=executor, jobs=jobs)

    def get_redis_conn(self) -> RedisConn:
        """Get redis connection."""
        return RedisConn(Redis(connection_pool=self.redis_pool))

    def submit_job(self, query_executor: QueryExecutor) -> bool:
        """Submit job."""
        future = self.executor.submit(query_executor.execute)
        self.jobs[query_executor.query_id] = future
        return True

    def cancel_job(self, query_id: str) -> bool:
        """Cancel job."""
        if query_id in self.jobs:
            if not self.jobs[query_id].done():
                self.jobs[query_id].cancel()

            del self.jobs[query_id]

            return True

        return False

state = AppState.new()

def get_state() -> AppState:
    """Get state."""
    return state

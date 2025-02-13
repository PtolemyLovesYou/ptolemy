"""Query engine."""

import logging
from concurrent.futures import ThreadPoolExecutor
import redis
from query_engine.consumer import Consumer
from query_engine.env_settings import (
    REDIS_HOST,
    REDIS_PORT,
    REDIS_DB,
    STREAM_NAME,
    GROUP_NAME,
)

logger = logging.getLogger(__name__)


def configure_logger():
    """Configure logger."""
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s - %(levelname)s - %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S",
    )


def main():
    """Main function."""

    consumer = Consumer(
        conn=redis.Redis(host=REDIS_HOST, port=REDIS_PORT, db=REDIS_DB),
        executor=ThreadPoolExecutor(max_workers=2),
        stream_name=STREAM_NAME,
        group_name=GROUP_NAME,
    )

    consumer.run()


if __name__ == "__main__":
    configure_logger()
    main()

"""Query engine."""

import logging
from concurrent.futures import ThreadPoolExecutor
from query_engine.consumer import Consumer
import redis

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
        conn=redis.Redis(host="redis", port=6379, db=0),
        executor=ThreadPoolExecutor(max_workers=2),
        stream_name="ptolemy:query",
        group_name="ptolemy:query",
    )

    consumer.run()


if __name__ == "__main__":
    configure_logger()
    main()

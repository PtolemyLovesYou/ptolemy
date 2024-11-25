"""Tvali main module."""

import argparse
import logging
import asyncio
from redis.asyncio import Redis

logging.basicConfig(level=logging.DEBUG)

parser = argparse.ArgumentParser()

parser.add_argument(
    "command", type=str, help="Command to execute", choices=["publisher", "observer"]
)

args = parser.parse_args()

if __name__ == "__main__":
    if args.command == "publisher":
        from tvali.publisher.main import listen

        asyncio.run(listen(Redis(host="redis", port=6379, db=0), "tvali_stream"))

    if args.command == "observer":
        from tvali.observer.server import serve

        asyncio.run(serve())

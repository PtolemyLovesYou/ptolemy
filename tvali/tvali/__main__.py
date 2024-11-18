import argparse
import asyncio
from redis.asyncio import Redis

parser = argparse.ArgumentParser()

parser.add_argument(
    "command", type=str, help="Command to execute", choices=["subscriber"]
)

args = parser.parse_args()

if __name__ == "__main__":
    if args.command == "subscriber":
        from tvali.subscriber.main import listen

        asyncio.run(listen(Redis(host="redis", port=6379, db=0), "tvali"))

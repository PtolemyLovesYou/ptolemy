"""Observer client."""

from typing import AsyncGenerator, List
import os
from contextlib import asynccontextmanager
from grpc import aio as grpc
from ...utils import Record
from ...proto import observer_pb2 as observer, observer_pb2_grpc as observer_grpc


@asynccontextmanager
async def get_grpc_channel() -> AsyncGenerator[grpc.Channel, None]:
    """Get gRPC channel."""
    async with grpc.insecure_channel(
        f"{os.getenv("OBSERVER_HOST")}:{os.getenv('OBSERVER_PORT')}"
    ) as f:
        yield f


async def push_records(records: List[Record]) -> None:
    """Push records."""
    async with get_grpc_channel() as channel:
        stub = observer_grpc.ObserverStub(channel)

        await stub.Publish(
            observer.PublishRequest(  # pylint: disable=no-member
                records=[r.proto() for r in records]
            )
        )

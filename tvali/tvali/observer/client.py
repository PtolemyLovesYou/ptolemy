"""Observer client."""

from typing import Optional, List
import os
from grpc import aio as grpc
from ..utils import Record
from ..proto import observer_pb2 as observer, observer_pb2_grpc as observer_grpc


def get_grpc_channel() -> grpc.Channel:
    """Get gRPC channel."""
    return grpc.insecure_channel(
        f"{os.getenv("OBSERVER_HOST")}:{os.getenv('OBSERVER_PORT')}"
    )


async def push_records(
    records: List[Record], channel: Optional[grpc.Channel] = None
) -> None:
    """Push records."""
    stub = observer_grpc.ObserverStub(channel or get_grpc_channel())

    rec = await stub.Publish(
        observer.PublishRequest(  # pylint: disable=no-member
            records=[r.proto() for r in records]
        )
    )
    print(rec)

"""Observer."""

from concurrent import futures
import asyncio
from datetime import datetime
from redis.asyncio import Redis
import grpc.aio as grpc
from ..proto import observer_pb2 as ob, observer_pb2_grpc as ob_grpc

client = Redis(host="redis", port=6379, db=0)
DEFAULT_STREAM = "tvali_stream"
MAX_STREAM_LENGTH = 1000000  # Maximum number of messages to keep in stream


async def publish_record(record: ob.Record) -> str:  # pylint: disable=no-member
    """Publish record."""
    message_data = {
        "data": record.SerializeToString(),
        "timestamp": str(datetime.now().timestamp()),
    }

    return await client.xadd(
        name=DEFAULT_STREAM,
        fields=message_data,
        maxlen=MAX_STREAM_LENGTH,
        approximate=True,
    )


class ObserverServicer(ob_grpc.ObserverServicer):
    """Observer Servicer."""

    async def Publish(  # pylint: disable=invalid-overridden-method
        self,
        request: ob.PublishRequest,  # pylint: disable=no-member
        context: grpc.ServicerContext,
    ) -> ob.PublishResponse:  # pylint: disable=no-member
        await asyncio.gather(*[publish_record(r) for r in request.records])

        return ob.PublishResponse(  # pylint: disable=no-member
            successful=True, message=""
        )


async def serve():
    """Serve observer server."""
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    ob_grpc.add_ObserverServicer_to_server(ObserverServicer(), server)
    server.add_insecure_port("[::]:50051")
    await server.start()
    await server.wait_for_termination()

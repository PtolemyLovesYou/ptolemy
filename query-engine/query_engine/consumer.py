"""Event consumer with structured logging."""

import multiprocessing
import logging
from typing import Literal, Optional, Self, List, Union
from concurrent.futures import ThreadPoolExecutor, Future
from pydantic import BaseModel, ConfigDict, model_validator, field_validator
import redis
from .query_executor import QueryExecutor


# Configure logging
logger = logging.getLogger(__name__)


class Message(BaseModel):
    """Message."""

    action: Literal["query", "cancel", "stop"]
    query_id: str
    allowed_workspace_ids: Optional[Union[str, List[str]]] = None
    query: Optional[str] = None
    batch_size: Optional[int] = None
    timeout_seconds: Optional[int] = None

    @field_validator("allowed_workspace_ids")
    @classmethod
    def validate_allowed_workspace_ids(cls, v):
        """Validate allowed workspace ids."""
        if v is None:
            return None
        if isinstance(v, str):
            return v.split(",")
        if isinstance(v, list):
            return v

        raise ValueError("allowed_workspace_ids must be a string or list")

    @model_validator(mode="after")
    def validate_action(self) -> Self:
        """Validate action."""
        if self.action not in ["query", "cancel", "stop"]:
            logger.error("Invalid message action", extra={"action": self.action})
            raise ValueError(f"Invalid action: {self.action}")

        if self.action == "query":
            if any(
                i is None
                for i in [self.query_id, self.allowed_workspace_ids, self.query]
            ):
                logger.error(
                    "Missing required fields for start action",
                    extra={
                        "query_id": self.query_id,
                        "allowed_workspace_ids": self.allowed_workspace_ids,
                        "query": self.query,
                    },
                )
                raise ValueError("Missing required fields for start action")

        if self.action == "cancel":
            if self.query_id is None:
                logger.error("Missing query_id for cancel action")
                raise ValueError("Missing required fields for cancel action")

        logger.debug(
            "Message validated successfully",
            extra={"action": self.action, "query_id": self.query_id},
        )
        return self


class Consumer(BaseModel):
    """Event consumer."""

    model_config = ConfigDict(arbitrary_types_allowed=True)
    executor: ThreadPoolExecutor
    conn: redis.Redis
    stream_name: str
    group_name: str

    consumer_name: str = f"consumer-{multiprocessing.current_process().pid}"
    block_ms: int = 5000  # Block for 5 seconds when reading

    cancelled: bool = False

    def _ensure_consumer_group(self):
        """Ensure consumer group exists, create if it doesn't."""
        try:
            self.conn.xgroup_create(
                name=self.stream_name,
                groupname=self.group_name,
                mkstream=True,
                id="0",  # Create from beginning of stream
            )
            logger.info(
                "Created consumer group",
                extra={"stream": self.stream_name, "group": self.group_name},
            )
        except redis.ResponseError as e:
            if "BUSYGROUP" not in str(e):  # Ignore if group already exists
                logger.error(
                    "Failed to create consumer group",
                    extra={
                        "stream": self.stream_name,
                        "group": self.group_name,
                        "error": str(e),
                    },
                )
                raise e
            logger.debug(
                "Consumer group already exists",
                extra={"stream": self.stream_name, "group": self.group_name},
            )

    def get_message(self) -> Message:
        """Get message from Redis stream.

        Reads from stream using XREADGROUP, which:
        1. Claims message for this consumer within the group
        2. Prevents other consumers in group from processing same message
        3. Requires explicit acknowledgment (XACK) after processing

        Returns:
            Message: Parsed message from stream
        """
        # Read new messages (> symbol)
        logger.debug("Attempting to read new messages from stream")
        messages = self.conn.xreadgroup(
            groupname=self.group_name,
            consumername=self.consumer_name,
            streams={self.stream_name: ">"},
            count=1,  # Get one message at a time
            block=self.block_ms,
        )

        # If no messages or empty result, try reading pending messages
        if not messages or not messages[0][1]:
            logger.debug("No new messages, checking pending messages")
            messages = self.conn.xreadgroup(
                groupname=self.group_name,
                consumername=self.consumer_name,
                streams={self.stream_name: "0"},  # Read from start for pending
                count=1,
            )

        if not messages or not messages[0][1]:  # Still no messages
            logger.debug("No messages available in stream")
            raise redis.ConnectionError("No messages available")

        # Parse message - we know messages[0][1] is not empty at this point
        _, message_list = messages[0]
        [message_id, message_dict] = message_list[0]

        try:
            # Convert redis byte strings to regular strings
            message_dict = {
                k.decode("utf-8"): v.decode("utf-8") for k, v in message_dict.items()
            }

            # Parse into Message model
            message = Message(**message_dict)

            # Acknowledge message
            self.conn.xack(self.stream_name, self.group_name, message_id)

            logger.info(
                "Successfully processed message from stream",
                extra={
                    "message_id": message_id,
                    "action": message.action,
                    "query_id": message.query_id,
                },
            )

            return message

        except Exception as e:
            # If parsing fails, acknowledge to prevent retry and raise
            self.conn.xack(self.stream_name, self.group_name, message_id)
            logger.error(
                "Failed to parse message",
                extra={"message_id": message_id, "error": str(e)},
                exc_info=True,
            )
            raise ValueError(f"Failed to parse message: {e}") from e

    def process_message(self, message: Message) -> Future:
        """Process a message by submitting it to the process pool.

        Args:
            message: Message to process

        Returns:
            Future: Handle to the processing task
        """
        logger.info(
            "Submitting message for processing",
            extra={"action": message.action, "query_id": message.query_id},
        )

        def _dummy_process(msg: Message) -> str:
            """Dummy processing function that simulates work."""
            wlogger = logging.getLogger(f"{__name__}.worker")
            wlogger.info(
                "Processing message",
                extra={"action": msg.action, "query_id": msg.query_id},
            )

            executor = QueryExecutor(
                logger=wlogger,
                query_id=msg.query_id,
                allowed_workspace_ids=msg.allowed_workspace_ids,
                query=msg.query
            )

            try:
                executor()
            except Exception as e:
                wlogger.error(
                    "Failed to process message: %s",
                    str(e),
                    exc_info=True,
                    extra={
                        "action": msg.action,
                        "query_id": msg.query_id,
                        "error": str(e),
                    },
                )
                raise

            wlogger.info(
                "Finished processing message",
                extra={"action": msg.action, "query_id": msg.query_id},
            )
            return f"Processed message {msg.query_id}"

        return self.executor.submit(_dummy_process, message)

    def stop(self) -> None:
        """Signal the consumer to stop processing messages."""
        logger.info("Stopping consumer")
        self.cancelled = True

    def run(self) -> None:
        """Run the consumer until stopped.

        This method will:
        1. Ensure the consumer group exists
        2. Process messages until stopped
        3. Handle shutdown gracefully
        """
        logger.info(
            "Starting consumer",
            extra={
                "stream": self.stream_name,
                "group": self.group_name,
                "consumer": self.consumer_name,
            },
        )

        self._ensure_consumer_group()
        active_futures: set[Future] = set()

        try:
            while not self.cancelled:
                try:
                    # Get and process message
                    message = self.get_message()

                    # Clean up completed futures
                    active_futures = {f for f in active_futures if not f.done()}

                    if message.action == "stop":
                        logger.info("Received stop message")
                        self.stop()
                        continue

                    # Submit for processing and track the future
                    future = self.process_message(message)
                    active_futures.add(future)

                except redis.ConnectionError:
                    # No messages available, continue polling
                    continue
                except Exception as e:  # pylint: disable=broad-except
                    logger.error(
                        "Error processing message",
                        exc_info=True,
                        extra={"error": str(e)},
                    )
                    continue

        finally:
            # Graceful shutdown
            logger.info("Initiating graceful shutdown")

            # Wait for active tasks to complete
            for future in active_futures:
                try:
                    future.result(timeout=10)  # Give each task up to 10 seconds
                except Exception as e:  # pylint: disable=broad-except
                    logger.error(
                        "Error during shutdown", exc_info=True, extra={"error": str(e)}
                    )

            # Cleanup
            logger.info("Shutting down executor and closing connections")
            self.executor.shutdown(wait=True)
            self.conn.close()
            logger.info("Consumer shutdown complete")

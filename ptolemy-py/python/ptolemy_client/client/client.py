"""Ptolemy Client."""

from __future__ import annotations

import logging
from pydantic import InstanceOf

from .base import PtolemyBase
from .._core import RecordExporter
from ..trace import Trace

logger = logging.getLogger(__name__)

class Ptolemy(PtolemyBase):
    """Ptolemy Client."""

    base_url: str

    client: InstanceOf[RecordExporter]

    def add_trace_blocking(self, trace: "Trace"):
        """Send trace."""
        # TODO: Batching, retries, etc.

        try:
            self.client.send_trace_blocking(trace)
        # Thrown when invalid trace is sent
        except AttributeError as e:
            logger.error("Invalid trace type: %s", trace.__class__.__name__)
        except ConnectionError as e:
            logger.error("Error sending trace %s: %s", trace.id_, e)

    async def add_trace(self, trace: "Trace"):
        """Send trace."""
        # TODO: Batching, retries, etc.

        try:
            await self.client.send_trace(trace)
        # Thrown when invalid trace is sent
        except AttributeError as e:
            logger.error("Invalid trace type: %s", trace.__class__.__name__)
        except ConnectionError as e:
            logger.error("Error sending trace %s: %s", trace.id_, e)

def connect(base_url: str) -> Ptolemy:
    """
    Connect to Ptolemy client.
    """
    client = RecordExporter(base_url)

    return Ptolemy(
        base_url=base_url,
        client=client,
    )

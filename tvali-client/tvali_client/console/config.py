"""Console config."""

import logging
from typing import Callable, Any
from functools import cached_property
from pydantic import Field
from tvali_utils import Tier, LogType
from ..config import TransportConfig

def default_message_formatter(data: dict, tier: Tier, record_type: LogType) -> dict:
    """Default message formatter for console transport."""
    return {
        "TvaliConsoleLogger": {
            "tier": tier.value,
            "record_type": record_type.value,
            "data": data
        }
    }

class ConsoleConfig(TransportConfig):
    """Console config."""
    log_level: int = Field(default=logging.INFO)
    message_formatter: Callable[[dict, Tier, LogType], Any] = Field(default=default_message_formatter)

    @cached_property
    def logger(self) -> logging.Logger:
        """Logger."""
        return logging.getLogger(__name__)

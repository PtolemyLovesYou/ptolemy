"""REST API client."""

from typing import ClassVar
from pydantic import create_model
from ..client import TvaliClient
from .log import TvaliLog
from .config import TvaliConfig


class Tvali(TvaliClient, TvaliConfig):
    """API client."""

    @property
    def transport_config(self) -> TvaliConfig:
        """Get transport config.

        Returns:
            TvaliConfig: Transport config
        """
        return TvaliConfig(
            **self.model_dump(
                include=TvaliConfig.model_fields.keys(),
            )
        )

    def log_class(self) -> type[TvaliLog]:
        return create_model(
            "APILog",
            __base__=TvaliLog,
            TRANSPORT_CONFIG=(ClassVar[TvaliConfig], self.transport_config),
        )

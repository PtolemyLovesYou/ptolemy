"""Tvali Client."""

from typing import Optional, ClassVar, Generic, TypeVar
from abc import ABC
from pydantic import BaseModel, create_model
from tvali_utils.types import Parameters
from tvali_utils.enums import Tier
from .log.core import Log
from .config import TransportConfig

TransportConfigType = TypeVar( # pylint: disable=invalid-name
    "TransportConfigType",
    bound=TransportConfig
    )

LogType = TypeVar( # pylint: disable=invalid-name
    "LogType",
    bound=Log
)

class TvaliClient(BaseModel, Generic[TransportConfigType, LogType], ABC):
    """Tvali client."""
    TRANSPORT_CONFIG_CLS: ClassVar[TransportConfigType]
    LOG_CLS: ClassVar[LogType]

    @property
    def transport_config(self) -> TransportConfig:
        """Get transport config.

        Returns:
            TvaliConfig: Transport config
        """
        config = self.TRANSPORT_CONFIG_CLS(
            **self.model_dump(
                include=self.TRANSPORT_CONFIG_CLS.model_fields.keys(),
            )
        )

        return config

    def trace(
        self,
        name: str,
        parameters: Optional[Parameters] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> Log:
        """Trace."""
        return (
            self
            .LOG_CLS
            .configure(Tier.SYSTEM, self.transport_config)
            .new(
                name=name,
                parameters=parameters,
                version=version,
                environment=environment,
            )
        )

def client_factory(
    name: str,
    log_cls: type[Log],
    transport_cls: type[TransportConfig]
    ) -> type[TvaliClient]:
    """Client factory."""
    return create_model(
        name,
        __base__=(TvaliClient[transport_cls, log_cls], transport_cls),
        LOG_CLS=(ClassVar[type[log_cls]], log_cls),
        TRANSPORT_CONFIG_CLS=(ClassVar[type[transport_cls]], transport_cls),
    )

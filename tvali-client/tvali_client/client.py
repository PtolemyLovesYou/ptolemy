"""Tvali Client."""

from typing import Optional, ClassVar, Generic, TypeVar
from pydantic import BaseModel, create_model
from tvali_utils import Parameters, Tier
from .log.core import Log
from .config import TransportConfig

TransportConfigType = TypeVar( # pylint: disable=invalid-name
    "TransportConfigType",
    bound=TransportConfig,
    covariant=True
    )

LogType = TypeVar( # pylint: disable=invalid-name
    "LogType",
    bound=Log,
    covariant=True
)

class TvaliClient(BaseModel, Generic[TransportConfigType, LogType]):
    """Tvali client."""
    TRANSPORT_CONFIG_CLS: ClassVar[type[TransportConfig]]
    LOG_CLS: ClassVar[type[LogType]]

    @property
    def transport_config(self) -> TransportConfigType:
        """
        Get the transport config.

        The transport config is a pydantic model created from the client's model fields that
        are specified in the transport config class.

        The client's model fields are filtered by the transport config class's `model_fields`
        attribute. The `model_fields` attribute is a dictionary of model field names to their
        corresponding types.

        Returns:
            TransportConfigType: The transport config.
        """
        config : TransportConfigType = self.TRANSPORT_CONFIG_CLS(
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
    ) -> LogType:
        """Trace."""
        return (
            self
            .LOG_CLS
            .configure(Tier.SYSTEM, self.transport_config)(
                name=name,
                parameters=parameters,
                version=version,
                environment=environment,
            )
        )

def client_factory(
    name: str,
    log_cls: type[LogType],
    transport_cls: type[TransportConfigType]
    ) -> type[TransportConfigType, TvaliClient[TransportConfigType, LogType]]: # type: ignore
    """Client factory."""
    return create_model(
        name,
        __base__=(transport_cls, TvaliClient,),
        TRANSPORT_CONFIG_CLS=(ClassVar[type[TransportConfigType]], transport_cls),
        LOG_CLS=(ClassVar[type[LogType]], log_cls),
    )

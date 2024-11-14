"""Tvali Client."""

from typing import Optional
from abc import ABC, abstractmethod
from pydantic import BaseModel
from tvali_utils.types import Parameters
from tvali_utils.enums import Tier
from .log.core import Log


class TvaliClient(BaseModel, ABC):
    """Tvali client."""

    @abstractmethod
    def log_class(self) -> type[Log]:
        """Get log class."""

    def trace(
        self,
        name: str,
        parameters: Optional[Parameters] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> Log:
        """Trace."""
        return (
            self.log_class()
            .tier(Tier.SYSTEM)
            .new(
                name=name,
                parameters=parameters,
                version=version,
                environment=environment,
            )
        )

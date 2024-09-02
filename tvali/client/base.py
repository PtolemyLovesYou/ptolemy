"""Base client class."""

from abc import ABC, abstractmethod
import logging
from functools import cached_property
from pydantic import BaseModel
from ..log.base import LogBase


class Client(BaseModel, ABC):
    """Client abstract class."""

    @cached_property
    def logger(self) -> logging.Logger:
        """The logger for this client.

        Returns a logger with the name of the module of the concrete client class.
        """
        return logging.getLogger(__name__)

    @abstractmethod
    async def log(self, log: LogBase) -> None:
        """Log a tvali LogBase object.

        This is an abstract method, concrete client classes must implement it.

        :param log: The log to log.
        :return: None
        """

    ...

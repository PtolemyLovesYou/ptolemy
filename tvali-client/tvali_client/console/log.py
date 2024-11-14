"""Console Log."""

from typing import ClassVar
import logging
from tvali_utils.enums import LogType
from .config import ConsoleConfig
from ..log.core import Log


class ConsoleLog(Log):
    """API Log."""

    TRANSPORT_CONFIG: ClassVar[ConsoleConfig]

    @property
    def _logger(self) -> logging.Logger:
        return self.TRANSPORT_CONFIG.logger

    def _log_data(self, data: dict, record_type: LogType) -> None:
        self.TRANSPORT_CONFIG.logger.log(
            self.TRANSPORT_CONFIG.log_level,
            self.TRANSPORT_CONFIG.message_formatter(data, self.TIER, record_type),
        )

    async def delete(self) -> None:
        self._logger.warning("ConsoleLogger cannot delete logs.")

    async def push_on_beginning(self) -> None:
        self._log_data(self.event_dict(), LogType.EVENT)

    async def push_on_ending(self) -> None:
        self._log_data(self.runtime_dict(), LogType.RUNTIME)

        for record_type, dicts_list in [
            (LogType.INPUT, self.inputs_dicts()),
            (LogType.OUTPUT, self.outputs_dicts()),
            (LogType.FEEDBACK, self.feedback_dicts()),
            (LogType.METADATA, self.metadata_dicts())
            ]:
            if dicts_list is not None:
                for d in dicts_list:
                    self._log_data(d, record_type)

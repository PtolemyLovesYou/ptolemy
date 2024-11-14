"""REST API client."""

from ...client import client_factory
from .log import TvaliLog
from .config import TvaliConfig

Tvali = client_factory("Tvali", TvaliLog, TvaliConfig)

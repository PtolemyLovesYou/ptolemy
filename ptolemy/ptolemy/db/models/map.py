"""Model map."""

from typing import Dict
from . import models, core
from ...utils import LogType, Tier

DB_OBJ_MAP: Dict[LogType, Dict[Tier, type[core.EventTable]]] = {
    log_type: {
        tier: getattr(models, f"{tier.capitalize()}{log_type.capitalize()}")
        for tier in Tier
    }
    for log_type in LogType
}

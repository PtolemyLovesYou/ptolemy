"""Model map."""

from typing import Dict
from tvali_utils import LogType, Tier
from . import models, core

DB_OBJ_MAP: Dict[LogType, Dict[Tier, type[core.EventTable]]] = {
    log_type: {
        tier: getattr(models, f"{tier.capitalize()}{log_type.capitalize()}")
        for tier in Tier
    }
    for log_type in LogType
}

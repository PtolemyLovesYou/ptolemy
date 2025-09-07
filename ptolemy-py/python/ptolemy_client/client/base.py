"""Ptolemy Client."""

from __future__ import annotations

from typing import Optional
from abc import ABC, abstractmethod
from uuid import UUID
from pydantic import BaseModel

from ..tier import Tier
from ..io import Parameters
from ..trace import Trace

class PtolemyBase(BaseModel, ABC):
    @abstractmethod
    def add_trace_blocking(self, trace: "Trace"): ...
    
    @abstractmethod
    async def add_trace(self, trace: "Trace"): ...
    
    def trace(
        self,
        subject_id: UUID,
        name: str,
        parameters: Optional[Parameters],
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> "Trace":
        """Create new trace."""

        return Trace(
            client=self,
            tier=Tier.SYSTEM,
            subject_id=subject_id,
            parent_id=subject_id,
            name=name,
            parameters=parameters,
            version=version,
            environment=environment,
        )

# Pydantic forward-ref resolution:
# Trace.client is annotated as type "PtolemyBase", but at import time Trace was
# defined before PtolemyBase was available in its namespace. Because we use
# `from __future__ import annotations`, Pydantic stores that annotation as a string
# and cannot resolve it automatically. After both classes are defined, we call
# `Trace.model_rebuild()` with a namespace mapping so Pydantic can resolve the
# forward reference and validate instances correctly at runtime.
Trace.model_rebuild()

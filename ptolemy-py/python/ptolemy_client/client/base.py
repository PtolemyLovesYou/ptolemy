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

# NOTE: Required to avoid Pydantic error
Trace.model_rebuild()

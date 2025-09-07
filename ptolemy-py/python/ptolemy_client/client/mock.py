"""Ptolemy Client."""

from __future__ import annotations

from typing import List
from pydantic import Field

from .base import PtolemyBase
from ..trace import Trace

class MockPtolemy(PtolemyBase):
    """Mock Ptolemy class."""

    traces: List["Trace"] = Field(default_factory=list)

    def add_trace_blocking(self, trace: "Trace"):
        self.traces.append(trace)

    async def add_trace(self, trace: "Trace"):
        self.traces.append(trace)

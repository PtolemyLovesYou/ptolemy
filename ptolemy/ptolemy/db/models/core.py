"""Abstract models."""

from typing import Dict, Any
import uuid
from datetime import datetime
from sqlalchemy import UUID, String, DateTime, JSON
from sqlalchemy.orm import Mapped, mapped_column
from ..session import Base


class EventTable(Base):
    """Base model for event tables."""

    __abstract__ = True

    id: Mapped[uuid.UUID] = mapped_column(UUID(), primary_key=True, default=uuid.uuid4)
    parent_id: Mapped[uuid.UUID] = mapped_column(UUID())

    created_at: Mapped[DateTime] = mapped_column(
        DateTime(), nullable=False, default=datetime.now()
    )


class Event(EventTable):
    """Event model."""

    __abstract__ = True

    name: Mapped[str] = mapped_column(String(), nullable=False)
    parameters: Mapped[Dict[str, Any]] = mapped_column(JSON(), nullable=True)
    environment: Mapped[str] = mapped_column(String(length=8), nullable=True)
    version: Mapped[str] = mapped_column(String(length=16), nullable=True)


class EventRuntime(EventTable):
    """Event runtime model."""

    __abstract__ = True

    start_time: Mapped[DateTime] = mapped_column(DateTime(), nullable=True)
    end_time: Mapped[DateTime] = mapped_column(DateTime(), nullable=True)
    error_type: Mapped[str] = mapped_column(String(), nullable=True)
    error_content: Mapped[str] = mapped_column(String(), nullable=True)


class EventInput(EventTable):
    """Event input model."""

    __abstract__ = True

    field_name: Mapped[str] = mapped_column(String(), nullable=False)
    field_value: Mapped[Any] = mapped_column(JSON(), nullable=True)


class EventOutput(EventTable):
    """Event output model."""

    __abstract__ = True

    field_name: Mapped[str] = mapped_column(String(), nullable=False)
    field_value: Mapped[Any] = mapped_column(JSON(), nullable=True)


class EventFeedback(EventTable):
    """Event feedback model."""

    __abstract__ = True

    field_name: Mapped[str] = mapped_column(String(), nullable=False)
    field_value: Mapped[Any] = mapped_column(JSON(), nullable=True)


class EventMetadata(EventTable):
    """Event metadata model."""

    __abstract__ = True

    field_name: Mapped[str] = mapped_column(String(), nullable=False)
    field_value: Mapped[str] = mapped_column(String(), nullable=True)

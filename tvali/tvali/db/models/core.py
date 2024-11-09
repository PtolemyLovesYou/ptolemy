"""Abstract models."""
from typing import Dict, Any
import uuid
from sqlalchemy import UUID, String, DateTime, JSON
from sqlalchemy.orm import Mapped, mapped_column
from ..session import Base

class Event(Base):
    """Event model."""
    __abstract__ = True

    id: Mapped[uuid.UUID] = mapped_column(UUID(), primary_key=True, default=uuid.uuid4)
    name: Mapped[str] = mapped_column(String(), nullable=False)
    parameters: Mapped[Dict[str, Any]] = mapped_column(JSON(), nullable=True)
    environment: Mapped[str] = mapped_column(String(length=8), nullable=False)
    version: Mapped[str] = mapped_column(String(length=16), nullable=False)


class EventRuntime(Base):
    """Event runtime model."""
    __abstract__ = True

    id: Mapped[uuid.UUID] = mapped_column(UUID(), primary_key=True, default=uuid.uuid4)
    start_time: Mapped[DateTime] = mapped_column(DateTime(), nullable=False)
    end_time: Mapped[DateTime] = mapped_column(DateTime(), nullable=False)
    error_type: Mapped[str] = mapped_column(String(), nullable=True)
    error_content: Mapped[str] = mapped_column(String(), nullable=True)


class EventInput(Base):
    """Event input model."""
    __abstract__ = True

    id: Mapped[uuid.UUID] = mapped_column(UUID(), primary_key=True, default=uuid.uuid4)
    field_name: Mapped[str] = mapped_column(String(), nullable=False)
    field_value: Mapped[Any] = mapped_column(JSON(), nullable=True)


class EventOutput(Base):
    """Event output model."""
    __abstract__ = True

    id: Mapped[uuid.UUID] = mapped_column(UUID(), primary_key=True, default=uuid.uuid4)
    field_name: Mapped[str] = mapped_column(String(), nullable=False)
    field_value: Mapped[Any] = mapped_column(JSON(), nullable=True)


class EventFeedback(Base):
    """Event feedback model."""
    __abstract__ = True

    id: Mapped[uuid.UUID] = mapped_column(UUID(), primary_key=True, default=uuid.uuid4)
    field_name: Mapped[str] = mapped_column(String(), nullable=False)
    field_value: Mapped[Any] = mapped_column(JSON(), nullable=True)


class EventMetadata(Base):
    """Event metadata model."""
    __abstract__ = True

    id: Mapped[uuid.UUID] = mapped_column(UUID(), primary_key=True, default=uuid.uuid4)
    field_name: Mapped[str] = mapped_column(String(), nullable=False)
    field_value: Mapped[str] = mapped_column(String(), nullable=True)

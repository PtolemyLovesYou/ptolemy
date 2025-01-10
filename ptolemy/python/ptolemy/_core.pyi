"""Header file for ptolemy core."""

# pylint: disable=unused-argument,missing-function-docstring,too-few-public-methods
from __future__ import annotations
from typing import Optional, Any, Dict, TypeVar, Final
from uuid import UUID
from datetime import datetime
from enum import Enum

T = TypeVar('T', bound=Enum)

class ApiKeyPermission(Enum):
    """API Key Permissions Enum."""
    READ_ONLY: Final[str] = "READ_ONLY"
    WRITE_ONLY: Final[str] = "WRITE_ONLY"
    READ_WRITE: Final[str] = "READ_WRITE"

class UserStatus(Enum):
    """User Status Enum."""
    ACTIVE: Final[str] = "ACTIVE"
    SUSPENDED: Final[str] = "SUSPENDED"

class WorkspaceRole(Enum):
    """Workspace Role Enum."""
    USER: Final[str] = "USER"
    MANAGER: Final[str] = "MANAGER"
    ADMIN: Final[str] = "ADMIN"

class Ptolemy:
    """Ptolemy Client."""

    def __init__(
        self,
        base_url: str,
        observer_url: str,
        workspace_name: str,
        autoflush: bool,
        batch_size: int,
    ) -> "Ptolemy": ...
    def trace(
        self,
        name: str,
        parameters: Optional[Dict[str, Any]] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> "Ptolemy": ...
    def child(
        self,
        name: str,
        parameters: Optional[Dict[str, Any]] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> "Ptolemy": ...
    def event(
        self,
        name: str,
        parameters: Optional[Dict[str, Any]] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> None: ...
    def runtime(
        self,
        start_time: float,
        end_time: float,
        error_type: Optional[str] = None,
        error_content: Optional[str] = None,
    ) -> None: ...
    def inputs(self, **kwds: Any) -> None: ...
    def outputs(self, **kwds: Any) -> None: ...
    def feedback(self, **kwds: Any) -> None: ...
    def metadata(self, **kwds: Any) -> None: ...
    def push_event(self) -> bool: ...
    def push_io(self) -> bool: ...
    def flush(self) -> bool: ...

class Workspace:
    """Workspace object."""
    id: UUID
    name: str
    description: Optional[str]
    archived: bool
    created_at: datetime
    updated_at: datetime

class User:
    """User object."""
    id: UUID
    username: str
    display_name: Optional[str]
    status: UserStatus
    is_admin: bool
    is_sysadmin: bool

class UserApiKey:
    """User API key."""
    id: UUID
    user_id: UUID
    name: str
    key_preview: str
    expires_at: Optional[datetime]

class ServiceApiKey:
    """Service API key."""
    id: UUID
    workspace_id: UUID
    name: str
    key_preview: str
    permissions: ApiKeyPermission
    expires_at: Optional[datetime]

class WorkspaceUser:
    """Workspace user."""
    workspace_id: UUID
    user_id: UUID
    role: WorkspaceRole

"""Header file for ptolemy core."""

# pylint: disable=unused-argument,missing-function-docstring,too-few-public-methods
from __future__ import annotations
from typing import Optional, Any, Dict, TypeVar, Final, List
from uuid import UUID
from datetime import datetime
from enum import Enum

T = TypeVar("T", bound=Enum)

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

class GraphQLClient:
    """GraphQL client."""
    def __init__(self, url: str): ...
    def create_workspace(
        self,
        user_id: UUID,
        name: str,
        admin_user_id: UUID,
        description: Optional[str] = None,
    ) -> Workspace: ...
    def delete_workspace(
        self,
        user_id: UUID,
        target_user_id: UUID,
        workspace_id: UUID,
        role: WorkspaceRole,
    ): ...
    def add_user_to_workspace(
        self,
        user_id: UUID,
        target_user_id: UUID,
        workspace_id: UUID,
        role: WorkspaceRole,
    ): ...
    def remove_user_from_workspace(
        self, user_id: UUID, workspace_id: UUID, target_user_id: UUID
    ): ...
    def change_user_workspace_role(
        self,
        user_id: UUID,
        target_user_id: UUID,
        workspace_id: UUID,
        role: WorkspaceRole,
    ): ...
    def create_service_api_key(
        self,
        user_id: UUID,
        workspace_id: UUID,
        name: str,
        permissions: ApiKeyPermission,
        valid_for: Optional[int] = None,
    ) -> str: ...
    def delete_service_api_key(
        self, user_id: UUID, workspace_id: UUID, api_key_id: UUID
    ): ...
    def get_workspace_service_api_keys(
        self, workspace_id: UUID
    ) -> List[ServiceApiKey]: ...
    def get_user_workspace_role(
        self, workspace_id: UUID, user_id: UUID
    ) -> WorkspaceRole: ...
    def get_workspace_users_by_name(
        self, workspace_name: str
    ) -> List[tuple[WorkspaceRole, User]]: ...
    def get_workspace_users(
        self, workspace_id: UUID
    ) -> List[tuple[WorkspaceRole, User]]: ...
    def create_user(
        self,
        user_id: UUID,
        username: str,
        password: str,
        is_admin: bool,
        is_sysadmin: bool,
        display_name: Optional[str] = None,
    ) -> User: ...
    def delete_user(self, user_id: UUID, target_user_id: UUID): ...
    def create_user_api_key(
        self, user_id: UUID, duration_days: Optional[int] = None
    ) -> str: ...
    def delete_user_api_key(self, user_id: UUID, api_key_id: UUID): ...
    def all_users(self) -> List[User]: ...
    def get_user_by_name(self, username: str) -> User: ...
    def get_user_workspaces(self, user_id: UUID) -> List[Workspace]: ...
    def get_user_workspaces_by_username(
        self, username: str
    ) -> List[tuple[WorkspaceRole, Workspace]]: ...
    def get_user_api_keys(self, user_id: UUID) -> List[UserApiKey]: ...
    def login(self, username: str, password: str) -> tuple[str, User]: ...

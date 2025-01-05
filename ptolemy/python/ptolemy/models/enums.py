"""Enums."""

from enum import StrEnum


class ApiKeyPermissionEnum(StrEnum):
    """API Key Permissions Enum."""

    READ_ONLY = "READ_ONLY"
    WRITE_ONLY = "WRITE_ONLY"
    READ_WRITE = "READ_WRITE"


class UserStatusEnum(StrEnum):
    """User Status Enum."""

    ACTIVE = "ACTIVE"
    SUSPENDED = "SUSPENDED"

class WorkspaceRoleEnum(StrEnum):
    """Workspace Role enum."""

    USER = "USER"
    MANAGER = "MANAGER"
    ADMIN = "ADMIN"

"""Enums."""

from enum import StrEnum


class UserRole(StrEnum):
    """User role."""

    USER = "USER"
    ADMIN = "ADMIN"
    SYSADMIN = "SYSADMIN"


class ApiKeyPermission(StrEnum):
    """API Key Permission Enum"""

    READ_ONLY = "READ_ONLY"
    WRITE_ONLY = "WRITE_ONLY"
    READ_WRITE = "READ_WRITE"


class WorkspaceRole(StrEnum):
    """Workspace role."""

    USER = "USER"
    MANAGER = "MANAGER"
    ADMIN = "ADMIN"

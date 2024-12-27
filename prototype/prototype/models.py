"""Models."""
from typing import Optional
from enum import StrEnum
from pydantic import BaseModel

class UserRole(StrEnum):
    """User role."""
    USER = "user"
    ADMIN = "admin"
    SYSADMIN = "sysadmin"

class User(BaseModel):
    """User model."""
    id: str
    username: str
    is_admin: bool
    is_sysadmin: bool
    display_name: Optional[str] = None
    status: str

    @property
    def role(self) -> UserRole:
        """User role."""
        if self.is_admin:
            return UserRole.ADMIN
        if self.is_sysadmin:
            return UserRole.SYSADMIN

        return UserRole.USER

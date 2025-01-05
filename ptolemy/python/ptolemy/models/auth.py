"""User model."""

from typing import Optional
from pydantic import BaseModel
from ..utils import ID


class User(BaseModel):
    """User model."""

    id: ID
    username: str
    is_admin: bool
    is_sysadmin: bool
    display_name: Optional[str] = None
    status: str

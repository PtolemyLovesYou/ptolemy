"""GraphQL Response model."""

from typing import Optional, List, Dict, Any, Type, TypeVar, Generic, ClassVar
import requests
from pydantic import (
    BaseModel,
    ConfigDict,
    AliasGenerator,
    alias_generators,
    ValidationError,
)
from .enums import UserStatusEnum, ApiKeyPermissionEnum, WorkspaceRoleEnum
from .auth import User, Workspace, ServiceApiKey, UserApiKey, WorkspaceUser
from ..utils import ID, Timestamp

T = TypeVar("T", bound=BaseModel)


class GQLResponseBase(BaseModel, Generic[T]):
    """GQL Response base class."""

    MODEL_CLS: ClassVar[Type[T]]

    model_config = ConfigDict(
        alias_generator=AliasGenerator(validation_alias=alias_generators.to_camel)
    )

    def to_model(self) -> T:
        """Convert to model."""
        try:
            return self.MODEL_CLS.model_validate(self.model_dump())
        except ValidationError as e:
            raise ValueError(
                f"Got a validation error: {e}. Check yo GQL query hoe!!!"
            ) from e


class GQLWorkspaceUser(GQLResponseBase[WorkspaceUser]):
    """GQL Workspace User."""

    MODEL_CLS = WorkspaceUser

    role: Optional[WorkspaceRoleEnum] = None
    user: Optional['GQLUser'] = None
    workspace: Optional['GQLWorkspace'] = None

    def get_user(self) -> 'GQLUser':
        """Get user."""
        if self.user is None:
            raise ValueError("No User fetched")

        return self.user

    def get_workspace(self) -> 'GQLWorkspace':
        """Get workspace."""
        if self.workspace is None:
            raise ValueError("No Workspace fetched")

        return self.workspace


class GQLServiceApiKey(GQLResponseBase[ServiceApiKey]):
    """GQL Service API key."""

    MODEL_CLS = ServiceApiKey

    id: Optional[ID] = None
    workspace_id: Optional[ID] = None
    name: Optional[str] = None
    key_preview: Optional[str] = None
    permissions: Optional[ApiKeyPermissionEnum] = None
    expires_at: Optional[Timestamp] = None


class GQLUserApiKey(GQLResponseBase[UserApiKey]):
    """GQL User API key."""

    MODEL_CLS = UserApiKey

    id: Optional[ID] = None
    user_id: Optional[ID] = None
    key_preview: Optional[str] = None
    expires_at: Optional[Timestamp] = None


class GQLWorkspace(GQLResponseBase[Workspace]):
    """GQL Workspace."""

    MODEL_CLS = Workspace

    id: Optional[ID] = None
    name: Optional[str] = None
    description: Optional[str] = None
    archived: Optional[bool] = None
    created_at: Optional[Timestamp] = None
    updated_at: Optional[Timestamp] = None
    users: Optional[List["GQLWorkspaceUser"]] = None
    service_api_keys: Optional[List["GQLServiceApiKey"]] = None


class GQLUser(GQLResponseBase[User]):
    """GQL User."""

    MODEL_CLS = User

    id: Optional[ID] = None
    name: Optional[ID] = None
    username: Optional[str] = None
    description: Optional[ID] = None
    archived: Optional[bool] = None
    created_at: Optional[Timestamp] = None
    updated_at: Optional[Timestamp] = None
    workspaces: Optional[List[GQLWorkspace]] = None
    status: Optional[UserStatusEnum] = None
    user_api_keys: Optional[List[GQLUserApiKey]] = None
    is_admin: Optional[bool] = None
    is_sysadmin: Optional[bool] = None


class GQLQuery(GQLResponseBase):
    """GraphQL Query model."""

    user: Optional[List[GQLUser]] = None
    workspace: Optional[List[GQLWorkspace]] = None

    def users(self) -> List[GQLUser]:
        """Users."""
        if self.user is None:
            raise ValueError("user is None.")

        return list(self.user)

    def workspaces(self) -> List[GQLWorkspace]:
        """Workspaces."""
        if self.workspace is None:
            raise ValueError("workspace is None.")

        return list(self.workspace)

    @classmethod
    def query(cls, query: str, variables: Dict[str, Any]) -> "GQLQuery":
        """Query GQL endpoint."""
        resp = requests.post(
            "http://localhost:8000/graphql",
            json={
                "query": query,
                "variables": variables,
            },
            timeout=5,
        )

        if not resp.ok:
            raise ValueError(f"GQL query failed: {resp.text}")

        data = resp.json().get("data")

        if data is None:
            raise ValueError(f"Data not in query response: {resp.text}")

        return cls(**data)

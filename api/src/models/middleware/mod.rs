use crate::{crypto::Claims, error::ApiError};
use uuid::Uuid;

pub type AuthResult<T> = Result<T, ApiError>;

#[derive(Clone, Debug)]
pub struct AccessAuditId(pub Uuid);

#[derive(Clone, Debug)]
pub struct WorkspacePermission {
    pub workspace: ptolemy::models::auth::Workspace,
    pub permissions: Option<ptolemy::models::enums::ApiKeyPermission>,
    pub role: Option<ptolemy::models::enums::WorkspaceRole>,
}

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub api_access_audit_log_id: Uuid,
    pub api_auth_audit_log_id: Uuid,
    pub user: Option<ptolemy::models::auth::User>,
    pub workspaces: Vec<WorkspacePermission>,
}

impl AuthContext {
    pub fn user(&self) -> Result<&ptolemy::models::auth::User, ApiError> {
        self.user.as_ref().ok_or(ApiError::InternalError)
    }

    pub fn can_create_delete_workspace(&self) -> bool {
        self.user.as_ref().map(|u| u.is_sysadmin).unwrap_or(false)
    }

    pub fn can_create_delete_user(&self, other_user_is_admin: bool, other_user_is_sysadmin: bool) -> bool {
        self.user.as_ref().map(|u| {
            !(other_user_is_admin || other_user_is_sysadmin)
            || u.is_sysadmin
        }).unwrap_or(false)
    }

    pub fn can_update_workspace(&self, workspace_id: uuid::Uuid) -> bool {
        for workspace in &self.workspaces {
            if workspace.workspace.id.as_uuid() == workspace_id {
                return match workspace.role {
                    Some(ptolemy::models::enums::WorkspaceRole::Admin) => true,
                    _ => false,
                }
            }
        }

        false
    }

    pub fn can_add_remove_update_user_to_workspace(&self, workspace_id: uuid::Uuid) -> bool {
        for workspace in &self.workspaces {
            if workspace.workspace.id.as_uuid() == workspace_id {
                return match workspace.role {
                    Some(ptolemy::models::enums::WorkspaceRole::Admin) => true,
                    _ => false,
                }
            }
        }

        false
    }

    pub fn can_create_delete_service_api_key(&self, workspace_id: uuid::Uuid) -> bool {
        for workspace in &self.workspaces {
            if workspace.workspace.id.as_uuid() == workspace_id {
                return match workspace.role {
                    Some(ptolemy::models::enums::WorkspaceRole::Admin) => true,
                    Some(ptolemy::models::enums::WorkspaceRole::Manager) => true,
                    _ => false,
                }
            }
        }

        false
    }

    pub fn can_read_workspace(&self, workspace_id: uuid::Uuid) -> bool {
        if self.user.is_some() {
            for workspace in &self.workspaces {
                return workspace.workspace.id.as_uuid() == workspace_id
            }
        }

        for workspace in &self.workspaces {
            if workspace.workspace.id.as_uuid() == workspace_id {
                return match workspace.permissions {
                    Some(ptolemy::models::enums::ApiKeyPermission::ReadOnly) => true,
                    Some(ptolemy::models::enums::ApiKeyPermission::ReadWrite) => true,
                    _ => false,
                }
            }
        }

        false
    }

    pub fn can_write_workspace(&self, workspace_id: uuid::Uuid) -> bool {
        for workspace in &self.workspaces {
            if workspace.workspace.id.as_uuid() == workspace_id {
                return match workspace.permissions {
                    Some(ptolemy::models::enums::ApiKeyPermission::ReadWrite) => true,
                    Some(ptolemy::models::enums::ApiKeyPermission::WriteOnly) => true,
                    _ => false,
                }
            }
        }

        false
    }
}

pub trait AuthHeader<T>: Clone + From<AuthResult<Option<T>>> + From<Option<AuthResult<T>>> {
    fn as_result(&self) -> Result<Option<&T>, ApiError>;

    fn ok(&self) -> Option<&T> {
        self.as_result().unwrap_or(None)
    }

    fn err(&self) -> Option<ApiError> {
        self.as_result().err()
    }

    fn undeclared(&self) -> bool {
        self.as_result().map(|o| o.is_none()).unwrap_or(false)
    }
}

macro_rules! auth_header {
    ($name:ident, $ty:ty) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            Ok($ty),
            Err(ApiError),
            Undeclared,
        }

        impl From<AuthResult<Option<$ty>>> for $name {
            fn from(result: AuthResult<Option<$ty>>) -> Self {
                match result {
                    AuthResult::Ok(Some(t)) => $name::Ok(t),
                    AuthResult::Ok(None) => $name::Undeclared,
                    AuthResult::Err(e) => $name::Err(e),
                }
            }
        }

        impl From<Option<AuthResult<$ty>>> for $name {
            fn from(result: Option<AuthResult<$ty>>) -> Self {
                Self::from(result.transpose())
            }
        }

        impl Into<AuthResult<Option<$ty>>> for $name {
            fn into(self) -> AuthResult<Option<$ty>> {
                match self {
                    $name::Ok(t) => AuthResult::Ok(Some(t)),
                    $name::Undeclared => AuthResult::Ok(None),
                    $name::Err(e) => AuthResult::Err(e),
                }
            }
        }

        impl AuthHeader<$ty> for $name {
            fn as_result(&self) -> Result<Option<&$ty>, ApiError> {
                match &self {
                    $name::Ok(t) => Ok(Some(t)),
                    $name::Undeclared => Ok(None),
                    $name::Err(e) => Err(e.clone()),
                }
            }
        }
    };
}

auth_header!(ApiKey, String);
auth_header!(JWT, Claims<Uuid>);

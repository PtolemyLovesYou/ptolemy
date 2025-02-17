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
    pub user: Option<ptolemy::models::auth::User>,
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
        match self.user.as_ref() {
            Some(u) => Ok(u),
            None => Err(ApiError::InternalError),
        }
    }

    pub fn can_create_delete_workspace(&self) -> bool {
        match &self.user {
            Some(u) => u.is_admin,
            None => false,
        }
    }

    pub fn can_create_delete_user(
        &self,
        other_user_is_admin: bool,
        other_user_is_sysadmin: bool,
    ) -> bool {
        match &self.user {
            Some(u) => !(other_user_is_admin || other_user_is_sysadmin) || u.is_sysadmin,
            None => false,
        }
    }

    pub fn can_update_workspace(&self, workspace_id: uuid::Uuid) -> bool {
        for workspace in &self.workspaces {
            if workspace.workspace.id.as_uuid() == workspace_id {
                return matches!(workspace.role, Some(ptolemy::models::enums::WorkspaceRole::Admin));
            }
        }

        false
    }

    pub fn can_add_remove_update_user_to_workspace(&self, workspace_id: uuid::Uuid) -> bool {
        for workspace in &self.workspaces {
            if workspace.workspace.id.as_uuid() == workspace_id {
                return matches!(workspace.role, Some(ptolemy::models::enums::WorkspaceRole::Admin));
            }
        }

        false
    }

    pub fn can_create_delete_service_api_key(&self, workspace_id: uuid::Uuid) -> bool {
        for workspace in &self.workspaces {
            if workspace.workspace.id.as_uuid() == workspace_id {
                return matches!(workspace.role, Some(ptolemy::models::enums::WorkspaceRole::Admin) | Some(ptolemy::models::enums::WorkspaceRole::Manager));
            }
        }

        false
    }

    pub fn can_read_workspace(&self, workspace_id: uuid::Uuid) -> bool {
        if self.user.is_some() {
            for workspace in &self.workspaces {
                if workspace.workspace.id.as_uuid() == workspace_id {
                    return true;
                }
            }
        }

        for workspace in &self.workspaces {
            if workspace.workspace.id.as_uuid() == workspace_id {
                return matches!(workspace.permissions, Some(ptolemy::models::enums::ApiKeyPermission::ReadOnly) | Some(ptolemy::models::enums::ApiKeyPermission::ReadWrite));
            }
        }

        false
    }

    pub fn can_write_workspace(&self, workspace_id: uuid::Uuid) -> bool {
        for workspace in &self.workspaces {
            if workspace.workspace.id.as_uuid() == workspace_id {
                return matches!(workspace.permissions, Some(ptolemy::models::enums::ApiKeyPermission::WriteOnly) | Some(ptolemy::models::enums::ApiKeyPermission::ReadWrite));
            }
        }

        false
    }
}

pub trait AuthHeader<T>: Clone + From<AuthResult<Option<T>>> + From<Option<AuthResult<T>>> {
    fn as_result(&self) -> Result<Option<&T>, ApiError>;

    fn ok(&self) -> Option<&T> {
        match self.as_result() {
            Ok(Some(t)) => Some(t),
            _ => None,
        }
    }

    fn err(&self) -> Option<ApiError> {
        match self.as_result() {
            Err(e) => Some(e),
            _ => None,
        }
    }

    fn undeclared(&self) -> bool {
        match self.as_result() {
            Ok(o) => o.is_none(),
            _ => false,
        }
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

        impl From<$name> for AuthResult<Option<$ty>> {
            fn from(header: $name) -> AuthResult<Option<$ty>> {
                match header {
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

impl ApiKey {
    pub fn content(&self) -> Result<String, ApiError> {
        match self {
            ApiKey::Ok(s) => Ok(s.clone()),
            _ => Err(ApiError::InternalError),
        }
    }

    pub fn api_key_type(&self) -> Result<ptolemy::generated::observer::ApiKeyType, ApiError> {
        match self {
            ApiKey::Ok(s) => {
                if s.starts_with("pt-pa") {
                    return Ok(ptolemy::generated::observer::ApiKeyType::User);
                }

                if s.starts_with("pt-sk") {
                    return Ok(ptolemy::generated::observer::ApiKeyType::Service);
                }

                Err(ApiError::InternalError)
            },
            _ => Err(ApiError::InternalError),
        }
    }
}

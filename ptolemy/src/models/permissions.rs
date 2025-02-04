use super::auth::{ServiceApiKey, User, UserApiKey};
use super::enums::{ApiKeyPermission, WorkspaceRole};

impl User {
    pub fn can_create_delete_user(
        &self,
        other_user_is_admin: bool,
        other_user_is_sysadmin: bool,
    ) -> bool {
        !(other_user_is_admin || other_user_is_sysadmin) || self.is_sysadmin
    }

    pub fn can_create_delete_workspace(&self) -> bool {
        self.is_admin
    }
}

impl WorkspaceRole {
    pub fn can_update_workspace(&self) -> bool {
        match self {
            WorkspaceRole::Admin => true,
            _ => false,
        }
    }

    pub fn can_add_user_to_workspace(&self) -> bool {
        match self {
            WorkspaceRole::Admin | WorkspaceRole::Manager => true,
            _ => false,
        }
    }

    pub fn can_remove_user_from_workspace(&self, other: &Self) -> bool {
        match self {
            WorkspaceRole::Admin => true,
            WorkspaceRole::Manager => match other {
                WorkspaceRole::User => true,
                _ => false,
            },
            WorkspaceRole::User => false,
        }
    }

    pub fn can_update_user_role(&self) -> bool {
        match self {
            WorkspaceRole::Admin => true,
            _ => false,
        }
    }

    pub fn can_create_delete_service_api_key(&self) -> bool {
        match self {
            WorkspaceRole::Admin | WorkspaceRole::User => true,
            _ => false,
        }
    }
}

impl ServiceApiKey {
    pub fn can_read(&self) -> bool {
        match self.permissions {
            ApiKeyPermission::ReadOnly | ApiKeyPermission::ReadWrite => true,
            _ => false,
        }
    }

    pub fn can_write(&self) -> bool {
        match self.permissions {
            ApiKeyPermission::ReadWrite | ApiKeyPermission::WriteOnly => true,
            _ => false,
        }
    }
}

impl UserApiKey {
    pub fn can_read(&self) -> bool {
        true
    }

    pub fn can_write(&self) -> bool {
        false
    }
}

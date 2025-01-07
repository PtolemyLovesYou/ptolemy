use chrono::NaiveDateTime;
use pyo3::prelude::*;
use crate::models::enums::{ApiKeyPermission, UserStatus, WorkspaceRole};
use uuid::Uuid;
use crate::types::PyUuid;

#[derive(Clone, Debug)]
#[pyclass(frozen)]
pub struct Workspace {
    id: Uuid,
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    description: Option<String>,
    #[pyo3(get)]
    archived: bool,
    #[pyo3(get)]
    created_at: NaiveDateTime,
    #[pyo3(get)]
    updated_at: NaiveDateTime,
}

#[pymethods]
impl Workspace {
    #[getter(id)]
    fn py_id(&self) -> PyResult<PyUuid> {
        PyUuid::try_from(self.id)
    }
}

impl Workspace {
    pub fn new(id: Uuid, name: String, description: Option<String>, archived: bool, created_at: NaiveDateTime, updated_at: NaiveDateTime) -> Self {
        Workspace {
            id,
            name,
            description,
            archived,
            created_at,
            updated_at,
        }
    }
}

#[derive(Clone, Debug)]
#[pyclass(frozen)]
pub struct User {
    id: Uuid,
    #[pyo3(get)]
    username: String,
    #[pyo3(get)]
    display_name: Option<String>,
    #[pyo3(get)]
    status: UserStatus,
    #[pyo3(get)]
    is_admin: bool,
    #[pyo3(get)]
    is_sysadmin: bool,
}

#[pymethods]
impl User {
    #[getter(id)]
    fn py_id(&self) -> PyResult<PyUuid> {
        PyUuid::try_from(self.id)
    }
}

impl User {
    pub fn new(id: Uuid, username: String, display_name: Option<String>, status: UserStatus, is_admin: bool, is_sysadmin: bool) -> Self {
        User {
            id,
            username,
            display_name,
            status,
            is_admin,
            is_sysadmin,
        }
    }
}

#[derive(Clone, Debug)]
#[pyclass(frozen)]
pub struct UserApiKey {
    id: Uuid,
    user_id: Uuid,
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    key_preview: String,
    #[pyo3(get)]
    expires_at: Option<NaiveDateTime>,
}

#[pymethods]
impl UserApiKey {
    #[getter(id)]
    fn py_id(&self) -> PyResult<PyUuid> {
        PyUuid::try_from(self.id)
    }

    #[getter(user_id)]
    fn py_user_id(&self) -> PyResult<PyUuid> {
        PyUuid::try_from(self.user_id)
    }
}

impl UserApiKey {
    pub fn new(id: Uuid, user_id: Uuid, name: String, key_preview: String, expires_at: Option<NaiveDateTime>) -> Self {
        UserApiKey {
            id,
            user_id,
            name,
            key_preview,
            expires_at,
        }
    }
}

#[derive(Clone, Debug)]
#[pyclass(frozen)]
pub struct ServiceApiKey {
    id: Uuid,
    workspace_id: Uuid,
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    key_preview: String,
    #[pyo3(get)]
    permissions: ApiKeyPermission,
    #[pyo3(get)]
    expires_at: Option<NaiveDateTime>,
}

#[pymethods]
impl ServiceApiKey {
    #[getter(id)]
    fn py_id(&self) -> PyResult<PyUuid> {
        PyUuid::try_from(self.id)
    }

    #[getter(workspace_id)]
    fn py_workspace_id(&self) -> PyResult<PyUuid> {
        PyUuid::try_from(self.workspace_id)
    }
}

impl ServiceApiKey {
    pub fn new(id: Uuid, workspace_id: Uuid, name: String, key_preview: String, permissions: ApiKeyPermission, expires_at: Option<NaiveDateTime>) -> Self {
        ServiceApiKey {
            id,
            workspace_id,
            name,
            key_preview,
            permissions,
            expires_at,
        }
    }
}

#[derive(Clone, Debug)]
#[pyclass(frozen)]
pub struct WorkspaceUser {
    id: Uuid,
    workspace_id: Uuid,
    user_id: Uuid,
    #[pyo3(get)]
    role: WorkspaceRole
}

#[pymethods]
impl WorkspaceUser {
    #[getter(id)]
    fn py_id(&self) -> PyResult<PyUuid> {
        PyUuid::try_from(self.id)
    }

    #[getter(workspace_id)]
    fn py_workspace_id(&self) -> PyResult<PyUuid> {
        PyUuid::try_from(self.workspace_id)
    }

    #[getter(user_id)]
    fn py_user_id(&self) -> PyResult<PyUuid> {
        PyUuid::try_from(self.user_id)
    }
}

impl WorkspaceUser {
    pub fn new(id: Uuid, workspace_id: Uuid, user_id: Uuid, role: WorkspaceRole) -> Self {
        WorkspaceUser {
            id,
            workspace_id,
            user_id,
            role,
        }
    }
}

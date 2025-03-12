use crate::enums::{api_key_permission::ApiKeyPermission, workspace_role::WorkspaceRole, user_status::UserStatus};
use crate::models::{PyServiceApiKey, PyUser, PyUserApiKey, PyWorkspace};
use crate::types::PyId;
use ptolemy::graphql::client::GraphQLClient;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass(frozen, name = "GraphQLClient")]
pub struct PyGraphQLClient(GraphQLClient);

#[pymethods]
impl PyGraphQLClient {
    #[new]
    #[pyo3(signature = (url, api_key, auth_method="api_key"))]
    pub fn new(url: String, api_key: &str, auth_method: &str) -> Self {
        Self(GraphQLClient::new(url, api_key, None, auth_method))
    }

    pub fn me(&self) -> PyResult<PyUser> {
        self.0
            .me()
            .map(|x| x.into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[pyo3(signature = (name, admin_user_id, description=None))]
    pub fn create_workspace(
        &self,
        name: String,
        admin_user_id: PyId,
        description: Option<String>,
    ) -> PyResult<PyWorkspace> {
        self.0
            .create_workspace(name, description, admin_user_id.into())
            .map(|x| x.into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn delete_workspace(&self, workspace_id: PyId) -> PyResult<()> {
        self.0
            .delete_workspace(workspace_id.into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn add_user_to_workspace(
        &self,
        user_id: PyId,
        workspace_id: PyId,
        role: WorkspaceRole,
    ) -> PyResult<()> {
        self.0
            .add_user_to_workspace(user_id.into(), workspace_id.into(), role.into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn remove_user_from_workspace(&self, workspace_id: PyId, user_id: PyId) -> PyResult<()> {
        self.0
            .remove_user_from_workspace(user_id.into(), workspace_id.into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn change_user_workspace_role(
        &self,
        user_id: PyId,
        workspace_id: PyId,
        role: WorkspaceRole,
    ) -> PyResult<()> {
        self.0
            .change_user_workspace_role(user_id.into(), workspace_id.into(), role.into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[pyo3(signature = (workspace_id, name, permissions, valid_for=None))]
    pub fn create_service_api_key(
        &self,
        workspace_id: PyId,
        name: String,
        permissions: ApiKeyPermission,
        valid_for: Option<isize>,
    ) -> PyResult<String> {
        self.0
            .create_service_api_key(workspace_id.into(), name, permissions.into(), valid_for)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn delete_service_api_key(&self, workspace_id: PyId, api_key_id: PyId) -> PyResult<()> {
        self.0
            .delete_service_api_key(workspace_id.into(), api_key_id.into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn get_workspace_service_api_keys(
        &self,
        workspace_id: PyId,
    ) -> PyResult<Vec<PyServiceApiKey>> {
        self.0
            .get_workspace_service_api_keys(workspace_id.into())
            .map(|x| x.into_iter().map(|x| x.into()).collect())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn get_user_workspace_role(
        &self,
        workspace_id: PyId,
        user_id: PyId,
    ) -> PyResult<WorkspaceRole> {
        self.0
            .get_user_workspace_role(workspace_id.into(), user_id.into())
            .map(|x| x.into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn get_workspace_users_by_name(
        &self,
        workspace_name: String,
    ) -> PyResult<Vec<(WorkspaceRole, PyUser)>> {
        self.0
            .get_workspace_users_by_name(workspace_name)
            .map(|data| {
                data.into_iter()
                    .map(|(role, user)| (role.into(), user.into()))
                    .collect()
            })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn get_workspace_users(
        &self,
        workspace_id: PyId,
    ) -> PyResult<Vec<(WorkspaceRole, PyUser)>> {
        self.0
            .get_workspace_users(workspace_id.into())
            .map(|data| {
                data.into_iter()
                    .map(|(role, user)| (role.into(), user.into()))
                    .collect()
            })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[pyo3(signature = (username, password, is_admin, is_sysadmin, display_name=None))]
    pub fn create_user(
        &self,
        username: String,
        password: String,
        is_admin: bool,
        is_sysadmin: bool,
        display_name: Option<String>,
    ) -> PyResult<PyUser> {
        self.0
            .create_user(username, password, is_admin, is_sysadmin, display_name)
            .map(|x| x.into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn delete_user(&self, user_id: PyId) -> PyResult<()> {
        self.0
            .delete_user(user_id.into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[pyo3(signature = (user_id, display_name=None, status=None, is_admin=None))]
    pub fn update_user(&self, user_id: PyId, display_name: Option<String>, status: Option<UserStatus>, is_admin: Option<bool>) -> PyResult<PyUser> {
        self.0
            .update_user(user_id.into(), display_name, status.map(|x| x.into()), is_admin)
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|u| u.into())
    }

    pub fn change_password(&self, user_id: PyId, current_password: String, new_password: String) -> PyResult<()> {
        self.0
            .change_user_password(user_id.into(), current_password, new_password)
            .map_err(|e| PyValueError::new_err(e.to_string()))
            .map(|_u| ())
    }

    #[pyo3(signature = (name, duration_days=None))]
    pub fn create_user_api_key(
        &self,
        name: String,
        duration_days: Option<isize>,
    ) -> PyResult<String> {
        self.0
            .create_user_api_key(name, duration_days)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn delete_user_api_key(&self, api_key_id: PyId) -> PyResult<()> {
        self.0
            .delete_user_api_key(api_key_id.into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn all_users(&self) -> PyResult<Vec<PyUser>> {
        self.0
            .all_users()
            .map(|x| x.into_iter().map(|x| x.into()).collect())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn get_user_by_name(&self, username: String) -> PyResult<PyUser> {
        self.0
            .get_user_by_name(username)
            .map(|x| x.into())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn get_user_workspaces(&self, user_id: PyId) -> PyResult<Vec<PyWorkspace>> {
        self.0
            .get_user_workspaces(user_id.into())
            .map(|x| x.into_iter().map(|x| x.into()).collect())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn get_user_workspaces_by_username(
        &self,
        username: String,
    ) -> PyResult<Vec<(WorkspaceRole, PyWorkspace)>> {
        self.0
            .get_user_workspaces_by_username(username)
            .map(|data| {
                data.into_iter()
                    .map(|(role, workspace)| (role.into(), workspace.into()))
                    .collect()
            })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    pub fn get_user_api_keys(&self, user_id: PyId) -> PyResult<Vec<PyUserApiKey>> {
        self.0
            .get_user_api_keys(user_id.into())
            .map(|x| x.into_iter().map(|x| x.into()).collect())
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

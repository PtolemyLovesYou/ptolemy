use crate::models::auth::{ServiceApiKey, User, UserApiKey};
use crate::models::enums::{ApiKeyPermission, WorkspaceRole};
use crate::models::id::Id;
use crate::{graphql::client::GraphQLClient, models::auth::Workspace};
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

    pub fn me(&self) -> PyResult<User> {
        Ok(self
            .0
            .me()
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    #[pyo3(signature = (name, admin_user_id, description=None))]
    pub fn create_workspace(
        &self,
        name: String,
        admin_user_id: Id,
        description: Option<String>,
    ) -> PyResult<Workspace> {
        Ok(self
            .0
            .create_workspace(name, description, admin_user_id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn delete_workspace(&self, workspace_id: Id) -> PyResult<()> {
        Ok(self
            .0
            .delete_workspace(workspace_id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn add_user_to_workspace(
        &self,
        user_id: Id,
        workspace_id: Id,
        role: WorkspaceRole,
    ) -> PyResult<()> {
        Ok(self
            .0
            .add_user_to_workspace(user_id, workspace_id, role)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn remove_user_from_workspace(&self, workspace_id: Id, user_id: Id) -> PyResult<()> {
        Ok(self
            .0
            .remove_user_from_workspace(user_id, workspace_id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn change_user_workspace_role(
        &self,
        user_id: Id,
        workspace_id: Id,
        role: WorkspaceRole,
    ) -> PyResult<()> {
        Ok(self
            .0
            .change_user_workspace_role(user_id, workspace_id, role)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    #[pyo3(signature = (workspace_id, name, permissions, valid_for=None))]
    pub fn create_service_api_key(
        &self,
        workspace_id: Id,
        name: String,
        permissions: ApiKeyPermission,
        valid_for: Option<isize>,
    ) -> PyResult<String> {
        Ok(self
            .0
            .create_service_api_key(workspace_id, name, permissions, valid_for)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn delete_service_api_key(&self, workspace_id: Id, api_key_id: Id) -> PyResult<()> {
        Ok(self
            .0
            .delete_service_api_key(workspace_id, api_key_id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn get_workspace_service_api_keys(&self, workspace_id: Id) -> PyResult<Vec<ServiceApiKey>> {
        Ok(self
            .0
            .get_workspace_service_api_keys(workspace_id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn get_user_workspace_role(
        &self,
        workspace_id: Id,
        user_id: Id,
    ) -> PyResult<WorkspaceRole> {
        Ok(self
            .0
            .get_user_workspace_role(workspace_id, user_id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn get_workspace_users_by_name(
        &self,
        workspace_name: String,
    ) -> PyResult<Vec<(WorkspaceRole, User)>> {
        Ok(self
            .0
            .get_workspace_users_by_name(workspace_name)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn get_workspace_users(&self, workspace_id: Id) -> PyResult<Vec<(WorkspaceRole, User)>> {
        Ok(self
            .0
            .get_workspace_users(workspace_id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    #[pyo3(signature = (username, password, is_admin, is_sysadmin, display_name=None))]
    pub fn create_user(
        &self,
        username: String,
        password: String,
        is_admin: bool,
        is_sysadmin: bool,
        display_name: Option<String>,
    ) -> PyResult<User> {
        Ok(self
            .0
            .create_user(username, password, is_admin, is_sysadmin, display_name)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn delete_user(&self, user_id: Id) -> PyResult<()> {
        Ok(self
            .0
            .delete_user(user_id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    #[pyo3(signature = (name, duration_days=None))]
    pub fn create_user_api_key(
        &self,
        name: String,
        duration_days: Option<isize>,
    ) -> PyResult<String> {
        Ok(self
            .0
            .create_user_api_key(name, duration_days)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn delete_user_api_key(&self, api_key_id: Id) -> PyResult<()> {
        Ok(self
            .0
            .delete_user_api_key(api_key_id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn all_users(&self) -> PyResult<Vec<User>> {
        Ok(self
            .0
            .all_users()
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn get_user_by_name(&self, username: String) -> PyResult<User> {
        Ok(self
            .0
            .get_user_by_name(username)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn get_user_workspaces(&self, user_id: Id) -> PyResult<Vec<Workspace>> {
        Ok(self
            .0
            .get_user_workspaces(user_id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn get_user_workspaces_by_username(
        &self,
        username: String,
    ) -> PyResult<Vec<(WorkspaceRole, Workspace)>> {
        Ok(self
            .0
            .get_user_workspaces_by_username(username)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn get_user_api_keys(&self, user_id: Id) -> PyResult<Vec<UserApiKey>> {
        Ok(self
            .0
            .get_user_api_keys(user_id)
            .map_err(|e| PyValueError::new_err(e.to_string()))?)
    }
}

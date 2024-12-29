"""Workspace management."""
from typing import Optional
from urllib.parse import urljoin
import requests
import streamlit as st
from .models import Workspace, UserRole, WorkspaceRole, ApiKeyPermission
from .user import get_users
from .auth import get_user_info
from .env_settings import API_URL

def delete_service_api_key(
    workspace_id: str,
    api_key_id: str,
    ):
    """Delete service api key."""
    resp = requests.delete(
        urljoin(API_URL, f"/workspace/{workspace_id}/api_key/{api_key_id}"),
        json={"user_id": get_user_info().id},
        timeout=5,
    )

    if not resp.ok:
        st.error(
            f"Failed to delete API key: {resp.text}"
)

def create_service_api_key(
    workspace_id: str,
    name: str,
    permission: ApiKeyPermission,
    duration: Optional[int] = None,
):
    """Create service api key."""
    data = {
        "user_id": get_user_info().id,
        "workspace_id": workspace_id,
        "name": name,
        "permission": permission,
        "duration": duration,
    }

    resp = requests.post(
        urljoin(API_URL, f"/workspace/{workspace_id}/api_key"),
        json=data,
        timeout=5,
    )

    if not resp.ok:
        st.toast(
            f"Failed to create API key: {resp.text}"
            )

def create_workspace(name: str, admin_id: Optional[str] = None, description: Optional[str] = None):
    """Create workspace."""
    body = {
        "user_id": get_user_info().id,
        "workspace_admin_user_id": admin_id or get_user_info().id,
        "workspace": {
            "name": name,
            "description": description,
        }
    }

    resp = requests.post(
        urljoin(API_URL, "/workspace"),
        json=body,
        timeout=5,
    )

    if not resp.ok:
        st.toast(
            "Failed to create workspace."
        )

def add_user_to_workspace(workspace_id: str, user_id: str, role: WorkspaceRole):
    """Add user to workspace."""
    resp = requests.post(
        urljoin(API_URL, f"/workspace/{workspace_id}/users/{user_id}"),
        json={"user_id": get_user_info().id, "role": role.capitalize()},
        timeout=5
    )

    if not resp.ok:
        st.toast(
            f"Failed to add user {user_id} to workspace {workspace_id}: {resp.text}"
            )
    else:
        st.rerun(scope="fragment")

def remove_user_from_workspace(workspace_id: str, user_id: str):
    """Remove user from workspace."""
    resp = requests.delete(
        urljoin(API_URL, f"/workspace/{workspace_id}/users/{user_id}"),
        timeout=5,
        json={"user_id": get_user_info().id},
    )

    if not resp.ok:
        st.toast(
            f"Failed to remove user {user_id} from workspace {workspace_id}: {resp.text}"
            )

def update_workspace_user_role(workspace_id: str, user_id: str, role: WorkspaceRole):
    """Update workspace user role."""
    resp = requests.put(
        urljoin(API_URL, f"/workspace/{workspace_id}/users/{user_id}"),
        json={"user_id": get_user_info().id, "role": role.capitalize()},
        timeout=5,
    )

    if not resp.ok:
        st.toast(
            f"Failed to update user {user_id} role in workspace {workspace_id}: {resp.text}"
            )

def add_user_to_workspace_form(workspace: Workspace):
    """Add user to workspace."""
    with st.form("add_user_to_workspace", clear_on_submit=True, border=False):
        valid_users = [
            i for i in get_users() if (
                i.role != UserRole.SYSADMIN
                and i.id not in [usr.id for usr in workspace.users]
                )
            ]

        sk_user = st.selectbox(
            "User",
            options=valid_users,
            format_func=lambda u: u.username,
            index=None,
            placeholder="Select user..."
        )

        sk_role = st.segmented_control(
            "Role",
            options=list(WorkspaceRole),
            default=WorkspaceRole.READER,
        )

        sk_submit = st.form_submit_button(label="Submit")

        if sk_submit:
            add_user_to_workspace(
                workspace.id,
                sk_user.id,
                sk_role
                )

def delete_workspace(workspace_id: str):
    """Delete workspace."""
    resp = requests.delete(
        urljoin(API_URL, f"/workspace/{workspace_id}"),
        json={"user_id": get_user_info().id},
        timeout=5,
    )

    if not resp.ok:
        st.toast(
            f"Failed to delete workspace {workspace_id}"
            )
    else:
        st.rerun(scope="fragment")

def create_workspace_form():
    """Create workspace form."""
    with st.form(
        "create_new_wk",
        border=False,
        clear_on_submit=True,
        enter_to_submit=False
        ):
        valid_admins = [i for i in get_users() if i.role != UserRole.SYSADMIN]
        sk_name = st.text_input("Name")
        sk_description = st.text_area("Description")
        sk_admin = st.pills(
            "Admin",
            options=valid_admins,
            format_func=lambda u: u.username,
        )
        sk_submit = st.form_submit_button(label="Create")

        if sk_submit:
            create_workspace(
                sk_name,
                admin_id=sk_admin.id if sk_admin else None,
                description=sk_description
            )

            st.rerun(scope="fragment")

def workspace_form(workspace: Workspace, user_workspace_role: WorkspaceRole):
    """Workspace form."""
    disabled = user_workspace_role not in (WorkspaceRole.ADMIN, WorkspaceRole.MANAGER)

    with st.form("wk_form", border=False, clear_on_submit=False):
        st.text_input("Name", value=workspace.name, disabled=True)

        st.text_area(
            "Description",
            value=workspace.description,
            key="wk_description",
            disabled=disabled
        )

        submit_wk = st.form_submit_button(
            label="Save",
            disabled=disabled
            )

        if submit_wk:
            st.rerun(scope="fragment")

    with st.popover("Delete Workspace", use_container_width=True, disabled=disabled):
        st.write("Are you sure you want to delete this workspace?")
        delete_wk_button = st.button("Delete", disabled=disabled)
        if delete_wk_button:
            delete_workspace(workspace.id)

@st.fragment
def wk_user_management_form(workspace: Workspace, user_workspace_role: WorkspaceRole):
    """Workspace user management form"""
    disabled = user_workspace_role not in (WorkspaceRole.ADMIN, WorkspaceRole.MANAGER)
    users = workspace.users

    with st.popover("Add User to Workspace", use_container_width=True, disabled=disabled):
        add_user_to_workspace_form(workspace)

    with st.form("Workspace User Management", clear_on_submit=True, border=False):
        wk_users = st.data_editor(
            [
                {
                    "username": u.username,
                    "role": u.workspace_role(workspace.id),
                    "delete": False
                    } for u in users
                ],
            use_container_width=True,
            column_config={
                "username": st.column_config.TextColumn(disabled=True),
                "role": st.column_config.SelectboxColumn(
                    options=list(WorkspaceRole),
                    disabled=disabled,
                    required=True,
                    ),
                "delete": st.column_config.CheckboxColumn(
                    disabled=disabled,
                    required=True,
                    ),
                },
            )

        submit_wk_users = st.form_submit_button(label="Save", disabled=disabled)

        if submit_wk_users:
            for user, user_row in zip(users, wk_users):
                if user_row["delete"]:
                    remove_user_from_workspace(workspace.id, user.id)
                    continue

                if user_row["role"] != user.role:
                    update_workspace_user_role(workspace.id, user.id, user_row["role"])

            st.rerun(scope="fragment")

@st.fragment
def service_api_key_management_form(workspace: Workspace, user_workspace_role: WorkspaceRole):
    """Service API Key Management"""
    disabled = user_workspace_role not in (WorkspaceRole.ADMIN, WorkspaceRole.MANAGER)

    with st.popover("New API Key", use_container_width=True, disabled=disabled):
        with st.form("new_api_key", clear_on_submit=True, border=False):
            new_api_key_name = st.text_input("Name")
            new_api_key_duration = st.slider(
                "Duration (Days)",
                min_value=1,
                max_value=365,
                value=30,
                step=1
                )

            new_api_key_permission = st.segmented_control(
                "Permission",
                options=list(ApiKeyPermission),
                default=ApiKeyPermission.READ_ONLY
                )

            submit_new_api_key = st.form_submit_button(label="Create")
            if submit_new_api_key:
                create_service_api_key(
                    workspace.id,
                    new_api_key_name,
                    new_api_key_permission,
                    duration=new_api_key_duration,
                )

    keys = workspace.api_keys

    with st.form("Service API Key Management", clear_on_submit=True, border=False):
        api_keys = st.data_editor(
            [
                {
                    "name": k.name,
                    "permission": k.permissions,
                    "expires_at": k.expires_at,
                    "delete": False
                    } for k in keys
                ],
            use_container_width=True,
            column_config={
                "name": st.column_config.TextColumn(disabled=True),
                "permission": st.column_config.SelectboxColumn(
                    options=list(ApiKeyPermission),
                    disabled=True,
                    ),
                "expires_at": st.column_config.DatetimeColumn(
                    disabled=True,
                ),
                "delete": st.column_config.CheckboxColumn(
                    disabled=disabled,
                    required=True,
                    ),
                },
            )

        submit_api_keys = st.form_submit_button(label="Save", disabled=disabled)

        if submit_api_keys:
            for key, key_row in zip(keys, api_keys):
                if key_row["delete"]:
                    delete_service_api_key(workspace.id, key.id)

            st.rerun(scope='fragment')

@st.fragment
def wk_management_view():
    """Get workspace management view."""
    wk_selection_col, new_col = st.columns([3, 1])

    with wk_selection_col:
        selected_workspace = st.selectbox(
            "Select workspace",
            options=get_user_info().workspaces,
            format_func=lambda i: i.name,
            placeholder="Select Workspace",
            label_visibility="collapsed",
            index=None,
            )

    with new_col:
        with st.popover(r"\+", use_container_width=True):
            create_workspace_form()

    with st.container(border=True, height=560):
        if selected_workspace is None:
            pass
        else:
            user_workspace_role = get_user_info().workspace_role(selected_workspace.id)

            wk_mgmnt, wk_users, api_keys = st.tabs(
                ["Workspace", "Users", "Service API Keys"]
                )

            with wk_mgmnt:
                workspace_form(
                    selected_workspace,
                    user_workspace_role,
                    )

            with wk_users:
                wk_user_management_form(
                    selected_workspace,
                    user_workspace_role,
                    )

            with api_keys:
                service_api_key_management_form(
                    selected_workspace,
                    user_workspace_role,
                    )

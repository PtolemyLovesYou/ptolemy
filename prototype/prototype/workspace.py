"""Workspace management."""
from typing import Optional
from urllib.parse import urljoin
import requests
import streamlit as st
from .models import Workspace, UserRole, WorkspaceRole
from .user import get_users
from .auth import get_user_info
from .env_settings import API_URL

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

def add_user_to_workspace_form(workspace: Workspace):
    """Add user to workspace."""
    with st.form("add_user_to_workspace", clear_on_submit=True, border=False):
        valid_users = [
            i for i in get_users() if i.role != UserRole.SYSADMIN and i.id not in [
                usr.id for usr in workspace.users
                ]
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

def workspace_form(workspace: Workspace):
    """Workspace form."""
    with st.form("wk_form", border=False, clear_on_submit=False):
        wk_description = st.text_area(
            "Description",
            placeholder=workspace.description,
            key="wk_description"
        )

        wk_user_cols = st.columns([0.5, 0.7, 1])

        with wk_user_cols[0]:
            st.markdown("**USERNAME**")

        with wk_user_cols[1]:
            st.markdown("**ROLE**")

        with wk_user_cols[2]:
            st.markdown("**86**")

        wk_roles = list(WorkspaceRole)

        for user in workspace.users:
            with wk_user_cols[0]:
                st.text_input(
                    "username",
                    value=user.username,
                    disabled=True,
                    key=f"wk_user_{user.id}_username",
                    label_visibility="collapsed",
                )

            with wk_user_cols[1]:

                st.segmented_control(
                    "role",
                    options=wk_roles,
                    default=user.workspace_role(workspace.id),
                    disabled=False,
                    key=f"wk_user_{user.id}_role",
                    label_visibility="collapsed"
                )

            with wk_user_cols[2]:
                st.checkbox(
                    "delete",
                    disabled=False,
                    key=f"wk_user_{user.id}_delete",
                    label_visibility="collapsed"
                )

        submit_col, _ = st.columns([0.5, 4])
        with submit_col:
            submit_wk = st.form_submit_button(label="Save", use_container_width=True)

        if submit_wk:
            st.rerun(scope="fragment")

@st.fragment
def wk_management_view():
    """Get workspace management view."""
    select_workspace_col, add_user_col, add_wk_col, delete_wk_col = st.columns([4, 1, 1.5, 1.5])
    with select_workspace_col:
        workspace: Workspace = st.selectbox(
            "Select workspace",
            options=get_user_info().workspaces,
            label_visibility='collapsed',
            format_func=lambda w: w.name,
            index=None,
            placeholder="Select workspace..."
            )

    with add_user_col:
        with st.popover(
            "Add User",
            use_container_width=False,
            ):
            if workspace is not None:
                add_user_to_workspace_form(workspace)

    with add_wk_col:
        with st.popover("New Workspace", use_container_width=False):
            create_workspace_form()

    with delete_wk_col:
        delete_wk_button = st.button(
            "Delete Workspace",
            disabled=workspace is None,
            use_container_width=True,
        )

        if delete_wk_button:
            delete_workspace(workspace.id)

    if workspace:
        workspace_form(workspace)

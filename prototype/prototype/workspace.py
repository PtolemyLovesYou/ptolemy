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

@st.fragment
def wk_management_view():
    """Get workspace management view."""
    select_workspace_col, add_user_col, add_wk_col = st.columns([4, 1, 1.25])
    with select_workspace_col:
        workspace: Workspace = st.selectbox(
            "Select workspace",
            options=get_user_info().workspaces,
            label_visibility='collapsed',
            format_func=lambda w: w.name,
            )

    with add_user_col:
        with st.popover(
            "Add User",
            use_container_width=False,
            disabled=workspace is None
            ):
            st.write('create user here')

    with add_wk_col:
        with st.popover("New Workspace...", use_container_width=False):
            create_workspace_form()

    if workspace:
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
                    st.selectbox(
                        "username",
                        options=workspace.users,
                        index=workspace.users.index(user),
                        disabled=True,
                        format_func=lambda u: u.username,
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

            submit_cols = st.columns([0.25, 0.25, 2])
            with submit_cols[0]:
                submit_wk = st.form_submit_button(label="Save", use_container_width=True)
            with submit_cols[1]:
                delete_wk = st.form_submit_button(label="Delete", use_container_width=True)

            if submit_wk:
                st.rerun(scope="fragment")

            if delete_wk:
                delete_workspace(workspace.id)
                st.rerun(scope="fragment")

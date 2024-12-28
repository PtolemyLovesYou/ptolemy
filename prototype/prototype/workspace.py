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

@st.fragment
def wk_management_view():
    """Get workspace management view."""
    tabs_col, add_col = st.columns([4, 0.25])
    with tabs_col:
        workspace: Workspace = st.selectbox(
            "Select workspace",
            options=get_user_info().workspaces,
            label_visibility='collapsed',
            format_func=lambda w: w.name,
            )

    with add_col:
        create_workspace_popover = st.popover(r"\+", use_container_width=False)
        with create_workspace_popover:
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

    if workspace:
        with st.form("wk_form", border=False):
            st.text_input(
                "Name",
                value=workspace.name,
                disabled=True,
                key="wk_name"
            )

            wk_description = st.text_area(
                "Description",
                placeholder=workspace.description,
                key="wk_description"
            )

            wk_user_cols = st.columns([0.5, 0.5, 1])

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
                    st.selectbox(
                        "role",
                        options=wk_roles,
                        index=wk_roles.index(user.workspace_role(workspace.id)),
                        disabled=False,
                        key=f"wk_user_{user.id}_role",
                        label_visibility="collapsed",
                    )

                with wk_user_cols[2]:
                    st.checkbox(
                        "delete",
                        disabled=False,
                        key=f"wk_user_{user.id}_delete",
                        label_visibility="collapsed"
                    )

            st.form_submit_button(label="Save", on_click=lambda: wk_description)

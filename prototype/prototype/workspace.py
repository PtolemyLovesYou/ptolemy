"""Workspace management."""
from typing import List, Optional
from urllib.parse import urljoin
import requests
import streamlit as st
from .models import Workspace, UserRole, User
from .user import get_users
from .auth import get_user_info
from .env_settings import API_URL

def get_workspaces() -> List[Workspace]:
    """Get workspaces."""
    user = get_user_info()

    resp = requests.get(
        urljoin(API_URL, f"/workspace_user/user/{user.id}"),
        timeout=5,
    )

    if not resp.ok:
        st.toast(
            f"Failed to get workspaces: {resp.status_code}"
            )

        return []

    return [Workspace(**wk) for wk in resp.json()]

def get_users_in_workspace(workspace_id: str) -> List[User]:
    """Get users in workspace."""
    resp = requests.get(
        urljoin(API_URL, f"/workspace_user/workspace/{workspace_id}"),
        timeout=5,
    )

    if not resp.ok:
        st.toast(
            f"Failed to get users in workspace: {resp.status_code}"
            )

        return []

    return [User(**u) for u in resp.json()]

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
            options=get_workspaces(),
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
            # st.text_input(
            #     "ID",
            #     value=workspace.id,
            #     disabled=True,
            #     key="wk_id"
            # )

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

            users = get_users_in_workspace(workspace.id)

            st.form_submit_button(label="Save", on_click=lambda: wk_description)

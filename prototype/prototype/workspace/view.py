"""Workspace management view."""

import streamlit as st
from .workspace_management import workspace_form
from .user_management import wk_user_management_form
from .api_key_management import service_api_key_management_form
from ..client import get_client, current_user


def create_workspace_form():
    """Create workspace form."""
    client = get_client()
    current_usr = current_user()

    with st.form(
        "create_new_wk", border=False, clear_on_submit=True, enter_to_submit=False
    ):
        valid_admins = {u.username: u for u in client.all_users() if not u.is_sysadmin}
        sk_name = st.text_input("Name")
        sk_description = st.text_area("Description")
        sk_admin = st.pills(
            "Admin",
            options=valid_admins.keys(),
        )
        sk_submit = st.form_submit_button(label="Create")

        if sk_submit:
            try:
                client.create_workspace(
                    current_usr.id,
                    sk_name,
                    valid_admins[sk_admin].id if sk_admin else current_usr.id,
                    description=sk_description
                )
                st.rerun(scope="fragment")
            except ValueError as e:
                st.error(f"Failed to create workspace: {e}")


@st.fragment
def wk_management_view():
    """Get workspace management view."""
    wk_selection_col, new_col = st.columns([3, 1])
    client = get_client()
    current_usr = current_user()
    workspaces = client.get_user_workspaces(current_usr.id)

    with wk_selection_col:
        selected_workspace = st.selectbox(
            "Select workspace",
            options=workspaces,
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
            user_workspace_role = client.get_user_workspace_role(
                selected_workspace.id,
                current_usr.id
            )

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

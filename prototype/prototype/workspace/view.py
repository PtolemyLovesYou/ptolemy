"""Workspace management view."""

import streamlit as st
from .workspace_management import workspace_form
from .user_management import wk_user_management_form
from .api_key_management import service_api_key_management_form
from ..models import User, Workspace


def create_workspace_form():
    """Create workspace form."""
    with st.form(
        "create_new_wk", border=False, clear_on_submit=True, enter_to_submit=False
    ):
        valid_admins = [i for i in User.all() if not i.is_sysadmin]
        sk_name = st.text_input("Name")
        sk_description = st.text_area("Description")
        sk_admin = st.pills(
            "Admin",
            options=valid_admins,
            format_func=lambda u: u.username,
        )
        sk_submit = st.form_submit_button(label="Create")

        if sk_submit:
            wk = Workspace.create(
                sk_name,
                admin_id=sk_admin.id if sk_admin else None,
                description=sk_description,
            )

            if wk:
                st.rerun(scope="fragment")


@st.fragment
def wk_management_view():
    """Get workspace management view."""
    wk_selection_col, new_col = st.columns([3, 1])

    with wk_selection_col:
        selected_workspace = st.selectbox(
            "Select workspace",
            options=User.current_user().workspaces,
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
            user_workspace_role = User.current_user().workspace_role(
                selected_workspace.id
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

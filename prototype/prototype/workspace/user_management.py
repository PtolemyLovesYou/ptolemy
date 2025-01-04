"""Workspace user management."""

import streamlit as st
from ..models import Workspace, WorkspaceRole, UserRole, User


def add_user_to_workspace_form(workspace: Workspace):
    """Add user to workspace."""
    with st.form("add_user_to_workspace", clear_on_submit=True, border=False):
        valid_users = [
            i
            for i in User.all()
            if (
                i.role != UserRole.SYSADMIN
                and i.id not in [usr.id for usr in workspace.users]
            )
        ]

        sk_user = st.selectbox(
            "User",
            options=valid_users,
            format_func=lambda u: u.username,
            index=None,
            placeholder="Select user...",
        )

        sk_role = st.segmented_control(
            "Role",
            options=list(WorkspaceRole),
            default=WorkspaceRole.USER,
        )

        sk_submit = st.form_submit_button(label="Submit")

        if sk_submit:
            success = workspace.add_user(sk_user, sk_role)
            if success:
                st.rerun(scope="fragment")


@st.fragment
def wk_user_management_form(workspace: Workspace, user_workspace_role: WorkspaceRole):
    """Workspace user management form"""
    disabled = user_workspace_role not in (WorkspaceRole.ADMIN, WorkspaceRole.MANAGER)
    users = workspace.users

    with st.popover(
        "Add User to Workspace", use_container_width=True, disabled=disabled
    ):
        add_user_to_workspace_form(workspace)

    with st.form("Workspace User Management", clear_on_submit=True, border=False):
        wk_users = st.data_editor(
            [
                {
                    "username": u.username,
                    "role": u.role,
                    "delete": False,
                }
                for u in users
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
                    workspace.remove_user(user)
                    continue

                if user_row["role"] != user.role:
                    workspace.change_user_role(user, user_row["role"])

            st.rerun(scope="fragment")

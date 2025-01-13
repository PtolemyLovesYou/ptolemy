"""Workspace user management."""

import streamlit as st
from ptolemy import WorkspaceRole, Workspace
from ..client import get_client


def add_user_to_workspace_form(workspace: Workspace):
    """Add user to workspace."""
    client = get_client()

    with st.form("add_user_to_workspace", clear_on_submit=True, border=False):
        valid_users = {
            i.username: i
            for i in client.all_users()
            if (
                (not i.is_sysadmin)
                and i.id
                not in [usr.id for (_, usr) in client.get_workspace_users(workspace.id)]
            )
        }

        sk_user = st.selectbox(
            "User",
            options=valid_users.keys(),
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
            try:
                client.add_user_to_workspace(
                    valid_users[sk_user].id, workspace.id, sk_role
                )

                st.rerun(scope="fragment")
            except ValueError as e:
                st.error(f"Failed to add user to workspace: {e}")


@st.fragment
def wk_user_management_form(workspace: Workspace, user_workspace_role: WorkspaceRole):
    """Workspace user management form"""
    client = get_client()
    disabled = user_workspace_role not in (WorkspaceRole.ADMIN, WorkspaceRole.MANAGER)
    roles, users = zip(*client.get_workspace_users(workspace.id))

    with st.popover(
        "Add User to Workspace", use_container_width=True, disabled=disabled
    ):
        add_user_to_workspace_form(workspace)

    with st.form("Workspace User Management", clear_on_submit=True, border=False):
        wk_users = st.data_editor(
            [
                {
                    "username": u.username,
                    "role": r,
                    "delete": False,
                }
                for (r, u) in zip(roles, users)
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
                    client.remove_user_from_workspace(
                        workspace.id, user.id
                    )
                    continue

                client.change_user_workspace_role(
                    user.id, workspace.id, user_row["role"]
                )

            st.rerun(scope="fragment")

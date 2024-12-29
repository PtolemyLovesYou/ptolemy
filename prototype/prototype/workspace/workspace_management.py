"""Workspace management."""

import streamlit as st
from ..models import WorkspaceRole, Workspace


def workspace_form(workspace: Workspace, user_workspace_role: WorkspaceRole):
    """Workspace form."""
    disabled = user_workspace_role not in (WorkspaceRole.ADMIN, WorkspaceRole.MANAGER)

    with st.form("wk_form", border=False, clear_on_submit=False):
        st.text_input("Name", value=workspace.name, disabled=True)

        st.text_area(
            "Description",
            value=workspace.description,
            key="wk_description",
            disabled=disabled,
        )

        submit_wk = st.form_submit_button(label="Save", disabled=disabled)

        if submit_wk:
            st.rerun(scope="fragment")

    with st.popover("Delete Workspace", use_container_width=True, disabled=disabled):
        st.write("Are you sure you want to delete this workspace?")
        delete_wk_button = st.button("Delete", disabled=disabled)
        if delete_wk_button:
            success = workspace.delete()
            if success:
                st.rerun(scope="fragment")

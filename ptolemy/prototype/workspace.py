"""Workspace management."""
from typing import Any
import uuid
import streamlit as st

def get_workspaces() -> dict[str, dict[str, Any]]:
    """Get workspaces."""
    return {
        uuid.uuid4().hex: {
            "name": f"workspace{i}", "description": f"wk{i} description"
            } for i in range(5)
    }

@st.fragment
def wk_management_view():
    """Get workspace management view."""
    workspaces = get_workspaces()
    tabs_col, add_col = st.columns([4, 0.25])
    with tabs_col:
        selected_wk_settings_name = st.selectbox(
            "Select workspace",
            options=[i['name'] for i in workspaces.values()],
            label_visibility='collapsed'
            )

        selected_wk_settings_id = [
            k for k, v in workspaces.items() if v['name'] == selected_wk_settings_name
            ][0]

        selected_wk_settings_data = workspaces[selected_wk_settings_id]

    with add_col:
        create_workspace_popover = st.popover(r"\+", use_container_width=False)
        with create_workspace_popover:
            st.write("Create workspace")

    with st.form("wk_form", border=False):
        st.text_input(
            "ID",
            value=selected_wk_settings_id,
            disabled=True,
            key="wk_id"
        )

        st.text_input(
            "Name",
            value=selected_wk_settings_data['name'],
            disabled=True,
            key="wk_name"
        )

        wk_description = st.text_area(
            "Description",
            placeholder=selected_wk_settings_data['description'],
            key="wk_description"
        )

        st.form_submit_button(label="Save", on_click=lambda: wk_description)

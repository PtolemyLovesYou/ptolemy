"""Workspace management."""
from typing import List
from urllib.parse import urljoin
import requests
import streamlit as st
from .models import Workspace
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
            st.write("Create workspace")

    if workspace:
        with st.form("wk_form", border=False):
            st.text_input(
                "ID",
                value=workspace.id,
                disabled=True,
                key="wk_id"
            )

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

            st.form_submit_button(label="Save", on_click=lambda: wk_description)

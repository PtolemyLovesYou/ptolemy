"""Streamlit prototype app."""
from typing import Any
import uuid
import streamlit as st

st.set_page_config(layout="wide")

def get_workspaces() -> dict[str, dict[str, Any]]:
    """Get workspaces."""
    return {
        uuid.uuid4().hex: {
            "name": f"workspace{i}", "description": f"wk{i} description"
            } for i in range(5)
    }

def get_users() -> dict[str, dict[str, Any]]:
    """Get users."""
    user_list = [
        {
            "username": "besaleli",
            "display_name": "Raz Besaleli",
            "role": "admin",
            "status": "active"
            },
        {
            "username": "kalmysh",
            "display_name": "Anton Kalmysh",
            "role": "user",
            "status": "active"
            },
        {
            "username": "kobakhidze",
            "display_name": "Irakli Kobakhidze",
            "role": "user",
            "status": "suspended"
            }
    ]

    return {uuid.uuid4().hex: u for u in user_list}

def get_main(sidebar_container, main_container):
    """Get main."""
    with sidebar_container:
        st.write("Sidebar")

    with main_container:
        st.write("Main")

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

        st.form_submit_button(label="Save",on_click=lambda: None)


def usr_management_view():
    """Get user management view."""
    # create button
    create_user_button = st.popover(r"\+", use_container_width=False)

    with create_user_button:
        st.write("Create user")

    users = get_users()

    header_container = st.container()
    with header_container:
        header = st.columns([0.75, 1, 1, 1, 0.5])
        with header[0]:
            st.markdown("**USERNAME**")
        with header[1]:
            st.markdown("**NAME**")
        with header[2]:
            st.markdown("**ROLE**")
        with header[3]:
            st.markdown("**STATUS**")
        with header[4]:
            st.markdown("**ACTIONS**")

    for user in users.values():
        user_container = st.container(border=False)
        with user_container:
            cols = st.columns([0.75, 1, 1, 1, 0.5])

            with cols[0]:
                st.write(user['username'])
            with cols[1]:
                st.write(user['display_name'])
            with cols[2]:
                st.write(user['role'])
            with cols[3]:
                st.write(user['status'])
            with cols[4]:
                st.popover(r"\.\.\.", use_container_width=False)

def usr_ak_management_view():
    """Get user API key management view."""
    st.write("User API key management view.")

def get_settings(sidebar_container, main_container):
    """Get settings view."""
    with sidebar_container:
        settings_radio = st.radio(
            "Settings Radio",
            label_visibility="collapsed",
            options=[
                "API Keys",
                "Workspace Management",
                "User Management"
                ]
            )

    with main_container:
        if settings_radio == "API Keys":
            usr_ak_management_view()
        if settings_radio == "Workspace Management":
            wk_management_view()
        if settings_radio == "User Management":
            usr_management_view()

def get_sql(sidebar_container, main_container):
    """Get sql view."""
    with sidebar_container:
        st.write("SQL sidebar")

    with main_container:
        st.write("SQL main")

def get_data_explorer(sidebar_container, main_container):
    """Get data explorer view."""
    with sidebar_container:
        st.write("Data explorer sidebar")

    with main_container:
        st.write("Data explorer main")

def get_layout():
    """Get layout."""
    sidebar_column, main_column = st.columns([1, 3], border=False, vertical_alignment="bottom")

    with sidebar_column:
        selected_view = st.segmented_control(
                label="selected_view",
                label_visibility="collapsed",
                options=["Data Explorer", "SQL", "Settings"],
                default="Data Explorer",
                )

        sidebar_container = st.container(height=600, border=True)

    with main_column:
        main_container = st.container(height=650, border=True)

    if selected_view == 'Data Explorer':
        get_data_explorer(sidebar_container, main_container)

    if selected_view == 'SQL':
        get_sql(sidebar_container, main_container)

    if selected_view == 'Settings':
        get_settings(sidebar_container, main_container)

get_layout()

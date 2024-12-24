"""Streamlit prototype app."""
from typing import Any, Optional
import uuid
import streamlit as st
import requests

USER_ID = uuid.uuid4().hex

st.set_page_config(layout="wide", page_title="Ptolemy")

def get_workspaces() -> dict[str, dict[str, Any]]:
    """Get workspaces."""
    return {
        uuid.uuid4().hex: {
            "name": f"workspace{i}", "description": f"wk{i} description"
            } for i in range(5)
    }

def user_role(is_admin: bool, is_sysadmin: bool) -> str:
    """Get user role."""
    if is_admin:
        return "admin"
    if is_sysadmin:
        return "sysadmin"

    return "user"

def create_new_user(username: str, password: str, role: str, display_name: Optional[str] = None):
    """Create new user."""
    resp = requests.post(
        "http://localhost:8000/user",
        json={
            "username": username,
            "password": password,
            "is_admin": role == "admin",
            "is_sysadmin": role == "sysadmin",
            "display_name": display_name,
        },
        timeout=5,
    )

    resp.raise_for_status()

def delete_user(user_id: str):
    """Delete user."""
    resp = requests.delete(
        f"http://localhost:8000/user/{user_id}",
        timeout=5,
    )

    if not resp.ok:
        st.toast(
            f"Failed to delete user {user_id}"
            )

def get_users() -> dict[str, dict[str, Any]]:
    """Get users."""
    user_list = requests.get(
        "http://localhost:8000/user/all",
        timeout=10,
    ).json()

    user_dict = {}

    for u in user_list:
        user_dict[u['id']] = u
        user_dict[u['id']]['role'] = user_role(
            u['is_admin'], u['is_sysadmin']
            )

    return {u['id']: u for u in user_list}

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

        st.form_submit_button(label="Save", on_click=lambda: wk_description)


def usr_management_view():
    """Get user management view."""
    # header
    header_columns = st.columns([1, 1, 5])
    with header_columns[0]:
        create_user_button = st.popover(r"\+", use_container_width=False)

        with create_user_button:
            with st.form(
                "create_new_usr_form",
                border=False,
                enter_to_submit=False,
                clear_on_submit=True
                ):
                new_usr_username = st.text_input("Username")
                new_usr_password = st.text_input("Password")
                new_usr_display_name = st.text_input("Display name")
                new_usr_role = st.pills(
                    "Role",
                    options=["user", "admin", "sysadmin"],
                    default="user"
                )

                submit = st.form_submit_button(label="Create")

                if submit:
                    create_new_user(
                        new_usr_username,
                        new_usr_password,
                        new_usr_role,
                        display_name=new_usr_display_name
                    )

    users = get_users()

    header_container = st.container()
    with header_container:
        header = st.columns([0.75, 1, 0.6, 0.6, 0.25])
        with header[0]:
            st.markdown("**USERNAME**")
        with header[1]:
            st.markdown("**NAME**")
        with header[2]:
            st.markdown("**ROLE**")
        with header[3]:
            st.markdown("**STATUS**")
        with header[4]:
            st.markdown("**DEL**")

    with st.form("usr_management_form", border=False, enter_to_submit=False):
        for user_id, user in users.items():
            user_container = st.container(border=False)
            with user_container:
                cols = st.columns([0.75, 1, 0.6, 0.6, 0.25])

                with cols[0]:
                    st.text_input(
                        "username",
                        value=user['username'],
                        disabled=True,
                        key=f"user_username_{user_id}",
                        label_visibility='collapsed'
                        )
                with cols[1]:
                    st.text_input(
                        "display_name",
                        value=user['display_name'],
                        disabled=False,
                        key=f"user_display_name_{user_id}",
                        label_visibility='collapsed'
                    )
                with cols[2]:
                    st.selectbox(
                        label=f"user_role_{user_id}",
                        options=["admin", "sysadmin", "user"],
                        index=["admin", "sysadmin", "user"].index(user['role']),
                        disabled=False,
                        key=f"user_role_{user_id}",
                        label_visibility='collapsed'
                    )
                with cols[3]:
                    st.selectbox(
                        label=f"user_status_{user_id}",
                        options=["Active", "Suspended"],
                        index=["Active", "Suspended"].index(user['status']),
                        disabled=False,
                        key=f"user_status_{user_id}",
                        label_visibility='collapsed'
                    )
                with cols[4]:
                    st.checkbox(
                        label=f"user_delete_{user_id}",
                        disabled=False,
                        key=f"user_delete_{user_id}",
                        label_visibility='collapsed'
                    )

        def delete_users():
            for user_id in users.keys():
                if st.session_state[f"user_delete_{user_id}"]:
                    delete_user(user_id)

        st.form_submit_button(label="Save", on_click=delete_users)

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

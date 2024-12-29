"""User management."""
from typing import Optional, List
from urllib.parse import urljoin
import requests
import streamlit as st
from .env_settings import API_URL
from .models import User, UserRole

def create_new_user(username: str, password: str, role: str, display_name: Optional[str] = None):
    """Create new user."""
    user_id = User.current_user().id

    resp = requests.post(
        urljoin(API_URL, "/user"),
        json={
            "user_id": user_id,
            "user": {
                "username": username,
                "password": password,
                "is_admin": role == UserRole.ADMIN,
                "is_sysadmin": role == UserRole.SYSADMIN,
                "display_name": display_name,
                },
        },
        timeout=5,
    )

    # if status code is forbidden, toast
    if resp.status_code == 403:
        st.toast(
            f"Failed to create user {username}: Unauthorized"
            )

def delete_user(user_id: str):
    """Delete user."""
    resp = requests.delete(
        urljoin(API_URL, f"/user/{user_id}"),
        json={
            "user_id": User.current_user().id,
            },
        timeout=5,
    )

    if not resp.ok:
        st.toast(
            f"Failed to delete user {user_id}"
            )

def get_users() -> List[User]:
    """Get users."""
    user_list = requests.get(
        urljoin(API_URL, "/user/all"),
        timeout=10,
    ).json()

    return [User(**u) for u in user_list]

@st.fragment
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
            st.markdown("**86**")

    with st.form("usr_management_form", border=False, enter_to_submit=False):
        for user in users:
            user_container = st.container(border=False)
            with user_container:
                cols = st.columns([0.75, 1, 0.6, 0.6, 0.25])

                with cols[0]:
                    st.text_input(
                        "username",
                        value=user.username,
                        disabled=True,
                        key=f"user_username_{user.id}",
                        label_visibility='collapsed'
                        )
                with cols[1]:
                    st.text_input(
                        "display_name",
                        value=user.display_name,
                        disabled=user.role == UserRole.SYSADMIN,
                        key=f"user_display_name_{user.id}",
                        label_visibility='collapsed'
                    )
                with cols[2]:
                    st.selectbox(
                        label=f"user_role_{user.id}",
                        options=["admin", "sysadmin", "user"],
                        index=["admin", "sysadmin", "user"].index(user.role),
                        disabled=user.role == UserRole.SYSADMIN,
                        key=f"user_role_{user.id}",
                        label_visibility='collapsed'
                    )
                with cols[3]:
                    st.selectbox(
                        label=f"user_status_{user.id}",
                        options=["Active", "Suspended"],
                        index=["Active", "Suspended"].index(user.status),
                        disabled=user.role == UserRole.SYSADMIN,
                        key=f"user_status_{user.id}",
                        label_visibility='collapsed'
                    )
                with cols[4]:
                    st.checkbox(
                        label=f"user_delete_{user.id}",
                        disabled=(
                            user.role == UserRole.SYSADMIN
                            or user.id == User.current_user().id
                            or (
                                user.role == UserRole.ADMIN
                                and User.current_user().role == UserRole.ADMIN
                                )
                            ),
                        key=f"user_delete_{user.id}",
                        label_visibility='collapsed'
                    )

        def delete_users():
            for user in users:
                if st.session_state[f"user_delete_{user.id}"]:
                    delete_user(user.id)

        st.form_submit_button(label="Save", on_click=delete_users)

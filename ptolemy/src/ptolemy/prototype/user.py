"""User management."""
from typing import Optional
from enum import StrEnum
import requests
import streamlit as st
from pydantic import BaseModel

class UserRole(StrEnum):
    """User role."""
    USER = "user"
    ADMIN = "admin"
    SYSADMIN = "sysadmin"

class User(BaseModel):
    """User model."""
    id: str
    username: str
    is_admin: bool
    is_sysadmin: bool
    display_name: Optional[str] = None
    status: str

    @property
    def role(self) -> UserRole:
        """User role."""
        if self.is_admin:
            return UserRole.ADMIN
        if self.is_sysadmin:
            return UserRole.SYSADMIN

        return UserRole.USER

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

def get_users() -> dict[str, User]:
    """Get users."""
    user_list = requests.get(
        "http://localhost:8000/user/all",
        timeout=10,
    ).json()

    return {
        u['id']: User(**u) for u in user_list
        }

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
        for user_id, user in users.items():
            user_container = st.container(border=False)
            with user_container:
                cols = st.columns([0.75, 1, 0.6, 0.6, 0.25])

                with cols[0]:
                    st.text_input(
                        "username",
                        value=user.username,
                        disabled=True,
                        key=f"user_username_{user_id}",
                        label_visibility='collapsed'
                        )
                with cols[1]:
                    st.text_input(
                        "display_name",
                        value=user.display_name,
                        disabled=False,
                        key=f"user_display_name_{user_id}",
                        label_visibility='collapsed'
                    )
                with cols[2]:
                    st.selectbox(
                        label=f"user_role_{user_id}",
                        options=["admin", "sysadmin", "user"],
                        index=["admin", "sysadmin", "user"].index(user.role),
                        disabled=False,
                        key=f"user_role_{user_id}",
                        label_visibility='collapsed'
                    )
                with cols[3]:
                    st.selectbox(
                        label=f"user_status_{user_id}",
                        options=["Active", "Suspended"],
                        index=["Active", "Suspended"].index(user.status),
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

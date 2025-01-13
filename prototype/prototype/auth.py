"""Auth services."""

import streamlit as st
from .client import get_client


def login(username: str, password: str) -> bool:
    """Login."""
    client = get_client()

    try:
        _, user = client.login(username, password)

        st.session_state.authenticated = True
        st.session_state.user_info = user

        return True
    except ValueError:
        return False


def logout():
    """Logout user."""
    st.session_state.authenticated = False
    st.session_state.user_info = None


def get_login_layout():
    """Login layout."""

    _, logocol, _ = st.columns([1, 0.25, 1])
    with logocol:
        st.image("assets/logomark_lime.svg", use_container_width=True)

    _, logincol, _ = st.columns([1, 1, 1])

    with logincol:
        with st.form("login_form", border=True):
            username = st.text_input("Username", key="auth_username")
            password = st.text_input("Password", type="password", key="auth_password")
            submit = st.form_submit_button("Login")
            if submit:
                if login(username, password):
                    st.session_state.authenticated = True
                    st.rerun()
                else:
                    st.error("Invalid username or password")

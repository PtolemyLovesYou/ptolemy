"""Auth services."""
from urllib.parse import urljoin
import streamlit as st
import requests
from .models import User
from .env_settings import API_URL

def login(username: str, password: str) -> bool:
    """Login."""
    resp = requests.post(
        urljoin(API_URL, "/auth"),
        json={"username": username, "password": password},
        timeout=5,
    )

    if resp.ok:
        data = resp.json()
        st.session_state.authenticated = True
        st.session_state.user_info = User(**data)

        return True

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

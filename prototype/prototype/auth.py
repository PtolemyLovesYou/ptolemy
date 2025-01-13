"""Auth services."""

import streamlit as st
import requests
from ptolemy import GraphQLClient

def login(username: str, password: str):
    """Login."""
    try:
        resp = requests.post(
            "http://api:8000/auth",
            json={"username": username, "password": password},
            timeout=5
            )
        if not resp.ok:
            raise ValueError(f"Invalid username or password: {resp.text}")

        token = resp.json()["token"]
        client = GraphQLClient("http://api:8000/graphql", token)
        user = client.me()

        st.session_state.authenticated = True
        st.session_state.user_info = user
        st.session_state.client = client

        st.rerun()
    except ValueError as e:
        return st.error(e)


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
                login(username, password)

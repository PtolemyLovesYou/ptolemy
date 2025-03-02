"""Client."""

import streamlit as st
from ptolemy_client import GraphQLClient, User


def get_client() -> GraphQLClient:
    """Get client."""
    if st.session_state.client is None:
        raise ValueError("Session state is not initialized")

    return st.session_state.client


def current_user() -> User:
    """Get current user info."""
    if st.session_state.user_info is None:
        raise ValueError("User is not logged in")

    return st.session_state.user_info

"""Client."""

import streamlit as st
from ptolemy import GraphQLClient

def get_client() -> GraphQLClient:
    """Get client."""
    if st.session_state is None:
        raise ValueError("Session state is not initialized")

    return st.session_state.client

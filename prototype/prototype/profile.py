"""Profile management view."""
import streamlit as st
from .models import User

@st.fragment
def profile_view():
    """Profile view."""
    current_user = User.current_user()

    st.subheader("Profile")
    st.text_input("Username", value=current_user.username, disabled=True)
    st.text_input("Display Name", value=current_user.display_name, disabled=True)

    with st.form("API Keys", clear_on_submit=True):
        st.subheader("API Keys")
        st.form_submit_button("Save")

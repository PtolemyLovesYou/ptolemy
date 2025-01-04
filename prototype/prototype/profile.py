"""Profile management view."""

import streamlit as st
from .models import User, UserApiKey


@st.fragment
def profile_view():
    """Profile view."""
    current_user = User.current_user()

    st.subheader("Profile")
    st.text_input("Username", value=current_user.username, disabled=True)
    st.text_input("Display Name", value=current_user.display_name, disabled=True)

    st.subheader("API Keys")
    with st.container(border=False):
        with st.popover("Create API Key", use_container_width=True):
            with st.form("Create API Key", clear_on_submit=True, border=False):
                new_personal_api_key_name = st.text_input("Name")
                new_personal_api_key_duration = st.slider(
                    "Duration (Days)", min_value=1, max_value=365, value=30, step=1
                )
                create_user_api_key_button = st.form_submit_button("Create")

                if create_user_api_key_button:
                    new_api_key = UserApiKey.create(
                        new_personal_api_key_name,
                        duration=new_personal_api_key_duration,
                    )

                    if new_api_key:
                        st.write("Copy API key here:")
                        st.code(new_api_key)

        with st.form("API Keys", clear_on_submit=True, border=False):
            api_keys = User.current_user().api_keys
            api_keys_editor = st.data_editor(
                [
                    {
                        "name": i.name,
                        "key_preview": f"{i.key_preview}...",
                        "expires_at": i.expires_at,
                        "delete": False,
                    }
                    for i in api_keys
                ],
                column_config={
                    "name": st.column_config.TextColumn(disabled=True),
                    "key_preview": st.column_config.TextColumn(disabled=True),
                    "expires_at": st.column_config.DatetimeColumn(disabled=True),
                    "delete": st.column_config.CheckboxColumn(),
                },
                use_container_width=True,
            )

            update_user_api_key_button = st.form_submit_button("Save")

            if update_user_api_key_button:
                for key, row in zip(api_keys, api_keys_editor):
                    if row["delete"]:
                        key.delete()

                st.rerun(scope="fragment")

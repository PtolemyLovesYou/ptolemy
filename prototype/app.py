"""Streamlit prototype app."""
import uuid
import logging
import streamlit as st
from prototype.auth import logout, get_login_layout
from prototype.user import usr_management_view
from prototype.workspace import wk_management_view

USER_ID = uuid.uuid4().hex

logging.basicConfig(level=logging.DEBUG)

st.set_page_config(layout="wide", page_title="Ptolemy")

if 'authenticated' not in st.session_state:
    st.session_state.authenticated = False
if 'user_info' not in st.session_state:
    st.session_state.user_info = None

if st.session_state.authenticated:
    st.logo("assets/logomark_lime.svg")

@st.fragment
def usr_ak_management_view():
    """Get user API key management view."""
    st.write("User API key management view.")

@st.fragment
def get_ide_view():
    """Code container."""
    st.write("SQL IDE goes here")

@st.fragment
def get_event_explorer_view():
    """Event explorer container."""
    st.write("Event explorer goes here")

@st.fragment
def get_account_management_view():
    """Get account management view."""
    st.write("Account management view")

def get_layout():
    """Get layout."""
    if not st.session_state.authenticated:
        get_login_layout()
    else:
        sidebar_column, main_column = st.columns([1, 11], border=False, vertical_alignment="bottom")

        with sidebar_column:
            sidebar_container = st.container(height=650, border=False)
            with sidebar_container:
                event_explorer_button = st.button(
                    "",
                    use_container_width = True,
                    icon=":material/monitoring:"
                )

                code_button = st.button(
                    "",
                    use_container_width=True,
                    icon=":material/code:"
                )

                api_keys_button = st.button(
                    "",
                    use_container_width = True,
                    icon=":material/key:"
                )

                workspace_management_button = st.button(
                    "",
                    use_container_width = True,
                    icon=":material/workspaces:"
                )

                user_management_button = st.button(
                    "",
                    use_container_width = True,
                    icon=":material/group:"
                )

                # spacer
                st.container(height=257, border=False)

                account_management_button = st.button(
                    "",
                    use_container_width = True,
                    icon=":material/account_circle:"
                )

                st.button(
                    "",
                    use_container_width=True,
                    icon=":material/logout:",
                    on_click=logout,
                )

        with main_column:
            main_container = st.container(height=650, border=True)
            with main_container:
                if event_explorer_button:
                    get_event_explorer_view()
                elif code_button:
                    get_ide_view()
                elif api_keys_button:
                    usr_ak_management_view()
                elif workspace_management_button:
                    wk_management_view()
                elif user_management_button:
                    usr_management_view()
                elif account_management_button:
                    get_account_management_view()
                else:
                    get_event_explorer_view()

get_layout()

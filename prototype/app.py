"""Streamlit prototype app."""

from urllib.parse import urljoin
import streamlit as st
from prototype.auth import logout, get_login_layout
from prototype.user import usr_management_view
from prototype.workspace.view import wk_management_view
from prototype.profile import profile_view
from prototype.models import User
from prototype.sql_ide import get_ide_view
from prototype.env_settings import API_URL
from ptolemy import GraphQLClient

st.set_page_config(layout="wide", page_title="Ptolemy")

if "authenticated" not in st.session_state:
    st.session_state.authenticated = False
if "user_info" not in st.session_state:
    st.session_state.user_info = None
if "client" not in st.session_state:
    st.session_state.client = GraphQLClient(urljoin(API_URL, "/graphql"))

if st.session_state.authenticated:
    st.logo("assets/logomark_lime.svg")


@st.fragment
def usr_ak_management_view():
    """Get user API key management view."""
    st.write("User API key management view.")


@st.fragment
def get_event_explorer_view():
    """Event explorer container."""
    st.write("Event explorer goes here")


def get_layout():
    """Get layout."""
    if not st.session_state.authenticated:
        get_login_layout()
    else:
        user_info = User.current_user()

        sidebar_column, main_column = st.columns(
            [1, 11], border=False, vertical_alignment="bottom"
        )

        with sidebar_column:
            sidebar_container = st.container(height=650, border=False)
            with sidebar_container:
                event_explorer_button = st.button(
                    "", use_container_width=True, icon=":material/monitoring:"
                )

                code_button = st.button(
                    "", use_container_width=True, icon=":material/code:"
                )

                workspace_management_button = st.button(
                    "",
                    use_container_width=True,
                    icon=":material/workspaces:",
                    # Sysadmins can't manage workspaces
                    disabled=user_info.is_sysadmin,
                )

                user_management_button = st.button(
                    "",
                    use_container_width=True,
                    icon=":material/group:",
                    disabled=(not (user_info.is_admin or user_info.is_sysadmin)),
                )

                # spacer
                st.container(height=314, border=False)

                profile_button = st.button(
                    "", use_container_width=True, icon=":material/account_circle:"
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
                elif workspace_management_button:
                    wk_management_view()
                elif user_management_button:
                    usr_management_view()
                elif profile_button:
                    profile_view()
                else:
                    get_event_explorer_view()


get_layout()

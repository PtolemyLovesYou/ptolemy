"""Streamlit prototype app."""
import streamlit as st

st.set_page_config(layout="wide")

def get_main(sidebar_container, main_container):
    """Get main."""
    with sidebar_container:
        st.write("Sidebar")

    with main_container:
        st.write("Main")

def wk_management_view():
    """Get workspace management view."""
    st.write("Workspace management view.")

def usr_management_view():
    """Get user management view."""
    st.write("User management view.")

def usr_ak_management_view():
    """Get user API key management view."""
    st.write("User API key management view.")

def get_settings(sidebar_container, main_container):
    """Get settings view."""
    with sidebar_container:
        usr_ak_management = st.button(
            "API Keys",
            use_container_width=True,
            key="usr_ak_management"
            )
        wk_management = st.button(
            "Workspace Management",
            use_container_width=True,
            key="wk_management"
            )
        usr_management = st.button(
            "User Management",
            use_container_width=True,
            key="usr_management"
            )

    with main_container:
        if usr_ak_management:
            usr_ak_management_view()
        elif wk_management:
            wk_management_view()
        elif usr_management:
            usr_management_view()
        # Show api key management by default
        else:
            usr_ak_management_view()

def get_sql(sidebar_container, main_container):
    """Get sql view."""
    with sidebar_container:
        st.write("SQL sidebar")

    with main_container:
        st.write("SQL main")

def get_data_explorer(sidebar_container, main_container):
    """Get data explorer view."""
    with sidebar_container:
        st.write("Data explorer sidebar")

    with main_container:
        st.write("Data explorer main")

def get_layout():
    """Get layout."""
    sidebar_column, main_column = st.columns([1, 3], border=False, vertical_alignment="bottom")

    with sidebar_column:
        selected_view = st.segmented_control(
                label="selected_view",
                label_visibility="hidden",
                options=["Data Explorer", "SQL", "Settings"],
                default="Data Explorer",
                )

        sidebar_container = st.container(height=600, border=True)

    with main_column:
        main_container = st.container(height=650, border=True)

    if selected_view == 'Data Explorer':
        get_data_explorer(sidebar_container, main_container)

    if selected_view == 'SQL':
        get_sql(sidebar_container, main_container)

    if selected_view == 'Settings':
        get_settings(sidebar_container, main_container)

get_layout()

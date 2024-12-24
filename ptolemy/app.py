"""Streamlit prototype app."""
import streamlit as st

st.set_page_config(layout="wide")

def get_main(sidebar_container, main_container):
    """Get main."""
    with sidebar_container:
        st.write("Sidebar")

    with main_container:
        st.write("Main")

def get_admin(sidebar_container, main_container):
    """Get admin view."""
    with sidebar_container:
        st.write("Admin sidebar")

    with main_container:
        st.write("Admin main")

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
                options=["Data Explorer", "SQL", "Admin"],
                default="Data Explorer",
                )

        sidebar_container = st.container(height=600, border=True)

        # get_sidebar(selected_view)

    with main_column:
        main_container = st.container(height=650, border=True)
        # get_main(selected_view)

    if selected_view == 'Data Explorer':
        get_data_explorer(sidebar_container, main_container)

    if selected_view == 'SQL':
        get_sql(sidebar_container, main_container)

    if selected_view == 'Admin':
        get_admin(sidebar_container, main_container)

get_layout()

"""Streamlit prototype app."""
import streamlit as st

st.set_page_config(layout="wide")

def get_sidebar(selected_view: str):
    """Get sidebar."""
    container = st.container(height=600, border=True)
    with container:
        if selected_view == "Data Explorer":
            st.write("Data explorer sidebar")
        if selected_view == "SQL":
            st.write("SQL sidebar")
        if selected_view == "Admin":
            st.write("Admin sidebar")

def get_explorer_main():
    """Get explorer main view"""
    st.write("Data explorer main view")

def get_sql_main():
    """Get sql main view"""
    st.write("SQL main view")

def get_admin_main():
    """Get admin main."""
    st.write("Admin main view")

def get_main(selected_view: str):
    """Get main."""
    container = st.container(height=650, border=True)
    with container:
        if selected_view == "Data Explorer":
            get_explorer_main()
        if selected_view == "SQL":
            get_sql_main()
        if selected_view == "Admin":
            get_admin_main()

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

        get_sidebar(selected_view)

    with main_column:
        get_main(selected_view)

get_layout()

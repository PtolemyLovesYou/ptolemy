"""Layout components."""
import streamlit as st

def get_sidebar():
    """Get sidebar."""
    title_col1, titlecol2 = st.columns([0.7, 0.3], vertical_alignment="bottom")

    with title_col1:
        st.subheader("PTOLEMY")
    with titlecol2:
        st.text("v0.0.1")

    st.container(border=True, height=512+32-8-2)

def get_df_container():
    """Get df container."""
    df_container = st.dataframe({}, use_container_width=True, height=256+64)
    with df_container:
        pass

def get_main_container():
    """Get main container."""
    main_container = st.container(border=True, height=256)
    with main_container:
        pass

def get_layout_columns():
    """Get layout columns."""
    sidebar, main = st.columns([0.25, 0.75])

    with sidebar:
        get_sidebar()

    with main:
        get_main_container()
        get_df_container()

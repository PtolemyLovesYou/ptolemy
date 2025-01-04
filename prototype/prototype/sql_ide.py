"""SQL IDE view."""

import streamlit as st
from .models.duckdb import DuckDB

st.session_state.ide_data = None


def get_results():
    """Get results."""
    st.dataframe(st.session_state.ide_data, use_container_width=True, height=385)


@st.fragment
def get_ide_view():
    """Code container."""
    duck = DuckDB.init()
    cs = duck.conn.cursor()
    with st.form("ide", enter_to_submit=True, border=False):
        code = st.text_area("Code", label_visibility="collapsed", height=150)

        submit = st.form_submit_button(label="Run")

    if submit:
        try:
            result = cs.sql(code)
            error = None
        except Exception as e:  # pylint: disable=broad-except
            result = None
            error = e
        if error is not None:
            st.error(error)
        else:
            if result is not None:
                result = result.df()
                st.session_state.ide_data = result
                get_results()

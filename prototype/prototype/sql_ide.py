"""SQL IDE view."""

import streamlit as st
import pandas as pd
from ptolemy._core import query # pylint: disable=no-name-in-module,import-error
from .env_settings import API_URL


st.session_state.ide_data = None


def get_results():
    """Get results."""
    st.dataframe(st.session_state.ide_data, use_container_width=True, height=385)


@st.fragment
def get_ide_view():
    """Code container."""
    with st.form("ide", enter_to_submit=True, border=False):
        code = st.text_area("Code", label_visibility="collapsed", height=150)

        submit = st.form_submit_button(label="Run")

    if submit:
        try:
            result = query( # pylint: disable=protected-access,no-member
                API_URL,
                st.session_state.jwt_token,
                code,
                batch_size=256,
                timeout_seconds=30
            )

            if result:
                result = pd.concat(result)
            else:
                result = pd.DataFrame()

            error = None
        except Exception as e:  # pylint: disable=broad-except
            result = None
            error = e
        if error is not None:
            st.error(error)
        else:
            if result is not None:
                st.session_state.ide_data = result
                get_results()

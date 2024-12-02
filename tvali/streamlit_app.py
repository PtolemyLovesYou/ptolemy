"""Main."""

import streamlit as st
from tvali.streamlit.layout import get_layout_columns

st.set_page_config(
    page_title="Ptolemy",
    # page_icon=":chart_with_upwards_trend:",
    layout="wide",
)

get_layout_columns()

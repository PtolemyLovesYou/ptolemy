"""Layout components."""

import asyncio
import streamlit as st
from streamlit_monaco import st_monaco
import pandas as pd
from .crud import (
    run_sql_query,
    get_event_objects,
    get_event_parameters,
    get_event_io,
    get_event_details,
    get_tier_event_names,
    get_tier_environments,
    get_tier_versions
    )
from ..utils import Tier, LogType


def get_filters(sql_mode: bool):
    """Get filters."""
    container = st.container(border=True, height=512 - 32 - 2)

    with container:
        if sql_mode:
            pass
        else:
            # Tier slider
            st.session_state["filter.tier"] = st.select_slider(
                "TIER", options=[*Tier], value=Tier.SYSTEM, label_visibility='collapsed'
            )

            # Name slider
            st.session_state["filter.name"] = st.multiselect(
                "NAME", options=asyncio.run(get_tier_event_names()), placeholder="..."
            )

            # Environment slider
            st.session_state["filter.environment"] = st.pills(
                "ENVIRONMENT", options=asyncio.run(get_tier_environments()),
                selection_mode='single'
            )

            # Version slider
            st.session_state["filter.version"] = st.multiselect(
                "VERSION", options=asyncio.run(get_tier_versions()), placeholder="..."
            )

            # Parent ID
            st.session_state["filter.id"] = st.text_input("ID", placeholder="...")

            refresh_button = st.button("Refresh", use_container_width=True)

            if refresh_button:
                st.rerun()

    return container


def get_sidebar() -> bool:
    """Get sidebar."""
    title_col1, titlecol2 = st.columns([0.7, 0.3], vertical_alignment="bottom")

    with title_col1:
        st.subheader("PTOLEMY")
    with titlecol2:
        st.text("v0.0.1")

    sql_mode = st.toggle("SQL Mode", value=False)

    get_filters(sql_mode)

    return sql_mode


def get_dataframe(df: pd.DataFrame):
    """Get df container."""
    st.session_state["dataframe"] = st.dataframe(
        df,
        use_container_width=True,
        height=256 + 64,
        selection_mode="single-row",
        hide_index=True,
        on_select="rerun",
    )

    selection = st.session_state["dataframe"].selection["rows"]
    if selection:
        st.session_state["event_id"] = df.iloc[selection[0]]["id"]
    else:
        st.session_state["event_id"] = None

def get_event_info() -> dict:
    """Get event information."""
    eis = st.session_state["event_info_selection"]
    if eis == "PARAMS":
        return asyncio.run(get_event_parameters())
    if eis == "INPUTS":
        return asyncio.run(get_event_io(LogType.INPUT))
    if eis == "OUTPUTS":
        return asyncio.run(get_event_io(LogType.OUTPUT))
    if eis == "FEEDBACK":
        return asyncio.run(get_event_io(LogType.FEEDBACK))
    if eis == "METADATA":
        return asyncio.run(get_event_io(LogType.METADATA))

    raise ValueError(f"Invalid event info selection: {eis}")

def get_main_container():
    """Get main container."""
    event_info_column, record_info_column = st.columns([0.45, 0.55])
    with record_info_column:
        record_info_container = st.container(border=True, height=256)
        with record_info_container:
            if st.session_state.get("event_id"):
                st.session_state["event_info_selection"] = st.segmented_control(
                    "Event Info",
                    options=["PARAMS", "INPUTS", "OUTPUTS", "FEEDBACK", "METADATA"],
                    default="PARAMS",
                    label_visibility='collapsed',
                    selection_mode='single'
                )

                st.json(get_event_info())
            else:
                st.empty()

    with event_info_column:
        other_info_container = st.container(border=True, height=256)
        with other_info_container:
            if st.session_state.get("event_id"):
                st.markdown(asyncio.run(get_event_details()))
            else:
                st.empty()


def get_layout_columns():
    """Get layout columns."""
    sidebar, main = st.columns([0.25, 0.75])

    with sidebar:
        sql_mode = get_sidebar()

    with main:
        main_container = st.container(border=False, height=256)
        if "data" not in st.session_state:
            st.session_state["data"] = asyncio.run(get_event_objects())

        if sql_mode:
            get_dataframe(pd.DataFrame({}))
        else:
            get_dataframe(st.session_state["data"])

        with main_container:
            if sql_mode:
                code_content = st_monaco(language="sql", height=145)

                run_button = st.button("Run")
                if run_button:
                    st.session_state["data"] = asyncio.run(run_sql_query(code_content))
                    st.rerun()
            else:
                st.session_state["data"] = asyncio.run(get_event_objects())
                get_main_container()

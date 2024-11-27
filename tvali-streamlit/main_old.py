"""Main."""

from typing import Optional
from enum import StrEnum
import requests
import streamlit as st
import pandas as pd

st.session_state["record_info"] = None


def get_children() -> Optional[dict]:
    """Get children."""
    return st.session_state["children"]

def set_children(data: dict) -> None:
    """Set children."""
    st.session_state["children"] = data

def get_record_info() -> Optional[dict]:
    """Get record info."""
    return st.session_state["record_info"]


def get_record_data() -> Optional[dict]:
    """Get record data."""
    return st.session_state["record_data"]


def set_record_info(record_info: dict) -> None:
    """Set record info."""
    st.session_state["record_info"] = record_info


def set_record_data(record_data: dict) -> None:
    """Set record data."""
    st.session_state["record_data"] = record_data


class Tier(StrEnum):
    """Tier."""

    SYSTEM = "system"
    SUBSYSTEM = "subsystem"
    COMPONENT = "component"
    SUBCOMPONENT = "subcomponent"

    def child(self) -> Optional["Tier"]:
        if self == Tier.SYSTEM:
            return Tier.SUBSYSTEM
        if self == Tier.SUBSYSTEM:
            return Tier.COMPONENT
        if self == Tier.COMPONENT:
            return Tier.SUBCOMPONENT

        return None


def get_parameters(parent_id: str, tier_: Tier) -> dict:
    """Get parameters."""
    query = f"""
    query fetchParameters(
        $Id: ID
    ) {{
        {tier_}Events(
            filters:{{
                id: $Id
            }}
        ) {{
            parentId
            parameters
            inputs {{
                fieldName
                fieldValue
            }}
            outputs {{
                fieldName
                fieldValue
            }}
            feedback {{
                fieldName
                fieldValue
            }}
            metadata {{
                fieldName
                fieldValue
            }}
            runtime {{
                startTime
                endTime
                errorType
                errorContent
            }}
        }}
    }}
    """

    data = requests.post(
        "http://tvali:8000/graphql",
        json={"query": query, "variables": {"Id": parent_id}},
        timeout=30,
    ).json()

    return {
        "parameters": data["data"][f"{tier_}Events"][0]["parameters"],
        "inputs": {
            d["fieldName"]: d["fieldValue"]
            for d in data["data"][f"{tier_}Events"][0]["inputs"]
        },
        "outputs": {
            d["fieldName"]: d["fieldValue"]
            for d in data["data"][f"{tier_}Events"][0]["outputs"]
        },
        "feedback": {
            d["fieldName"]: d["fieldValue"]
            for d in data["data"][f"{tier_}Events"][0]["feedback"]
        },
        "metadata": {
            d["fieldName"]: d["fieldValue"]
            for d in data["data"][f"{tier_}Events"][0]["metadata"]
        },
        "runtime": data["data"][f"{tier_}Events"][0]["runtime"],
    }


def fetch_data(variables: dict, tier_: Tier) -> dict:
    """Get data."""
    query = f"""
    query fetchDataset(
        $Limit: Int,
        $Offset: Int,
        $Id: ID,
        $ParentId: ID,
        $Version: String,
        $Environment: String,
        $Name: String
        ) {{
        {tier_}Events(
            limit: $Limit
            offset: $Offset
            filters:{{
            environment: $Environment,
            id: $Id,
            parentId: $ParentId,
            name: $Name,
            version: $Version
            }}
        ) {{
            id
            name
            version
            environment
            parentId
        }}
    }}
    """

    return requests.post(
        "http://tvali:8000/graphql",
        json={"query": query, "variables": variables},
        timeout=30,
    ).json()


st.set_page_config(
    page_title="Ptolemy",
    # page_icon=":chart_with_upwards_trend:",
    layout="wide",
)

sidebar, main, details = st.columns([0.20, 0.60, 0.2])

with sidebar:
    title_col1, titlecol2 = st.columns([0.7, 0.3], vertical_alignment="bottom")

    with title_col1:
        st.subheader("PTOLEMY")
    with titlecol2:
        st.text("v0.0.1")

    filters_container = st.container(border=True, height=384 + 128 + 16 + 8)
    with filters_container:
        tier = st.select_slider(
            "Tier",
            options=[*Tier],
            value=Tier(st.query_params.get("tier", Tier.SYSTEM)),
            label_visibility="hidden",
        )

        parent_id_ = st.text_input("Parent ID", value=st.query_params.get("parent_id", None))

        name = st.pills(
            "Name",
            options=["foo"],
            selection_mode="single",
        )

        environment = st.pills(
            "Environment",
            options=["DEV", "PROD", "STAGE"],
            selection_mode="single",
        )

def get_dataframe():
    """Get dataframe."""
    params = {"Limit": 100, "Offset": 0, "Environment": environment, "Name": name}

    response = fetch_data({i: j for i, j in params.items() if j is not None}, tier)

    return pd.DataFrame(response["data"][f"{tier}Events"])


with main:
    charts_container = st.container(height=192, border=True)
    dataframe_container = st.container(border=False)

    df = get_dataframe()

    with dataframe_container:
        dataframe = st.dataframe(
            data=df,
            use_container_width=True,
            hide_index=True,
            selection_mode="single-row",
            height=384,
            on_select="rerun",
        )

    if dataframe.selection["rows"]:
        row_num = dataframe.selection["rows"][0]
        set_record_info(df.iloc[row_num])
        set_record_data(get_parameters(df.iloc[row_num]["id"], tier))
        if tier != Tier.SUBCOMPONENT:
            set_children(
                fetch_data(
                    {"ParentId": df.iloc[row_num]["id"]},
                    tier.child()
                    )["data"][f"{tier.child()}Events"]
                )

    with charts_container:
        if dataframe.selection["rows"]:
            facts = [f"Parent ID: {get_record_info()["parentId"]}"]

            if tier.child():
                facts.append(f"# children: {len(get_children())}")

            st.text("\n".join(facts))

with details:
    cont = st.container(height=192 + 384 + 16, border=True)
    with cont:
        if dataframe.selection["rows"]:
            for i in ["RUNTIME", "PARAMETERS", "INPUTS", "OUTPUTS", "FEEDBACK", "METADATA"]:
                st.text(i)
                st.json(get_record_data()[i.lower()])
        else:
            st.text("Select a row to see details...")

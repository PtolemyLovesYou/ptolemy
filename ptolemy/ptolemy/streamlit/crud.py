"""CRUD ops."""

import streamlit as st
from sqlalchemy import text, select
import pandas as pd
from ..db import session, models
from ..utils import LogType


async def get_tier_event_names():
    async with session.get_db() as db:
        result = await db.execute(
            select(
                models.DB_OBJ_MAP[LogType.EVENT][
                    st.session_state["filter.tier"]
                ].name.distinct()
            )
        )

    return result.scalars().all()


async def get_tier_environments():
    async with session.get_db() as db:
        result = await db.execute(
            select(
                models.DB_OBJ_MAP[LogType.EVENT][
                    st.session_state["filter.tier"]
                ].environment.distinct()
            )
        )

    return result.scalars().all()


async def get_tier_versions():
    async with session.get_db() as db:
        result = await db.execute(
            select(
                models.DB_OBJ_MAP[LogType.EVENT][
                    st.session_state["filter.tier"]
                ].version.distinct()
            )
        )

    return result.scalars().all()


async def run_sql_query(query: str):
    """Run SQL query."""
    async with session.get_db() as db:
        result = await db.execute(text(query))

    keys = result.keys()

    return pd.DataFrame(
        ({c: getattr(r, c) for c in keys} for r in result.fetchall()), columns=keys
    )


async def get_event_objects():
    model = models.DB_OBJ_MAP[LogType.EVENT][st.session_state["filter.tier"]]

    filters = []

    if st.session_state.get("filter.name"):
        filters += [model.name.in_(st.session_state["filter.name"])]
    if st.session_state.get("filter.environment"):
        filters += [model.environment == st.session_state["filter.environment"]]
    if st.session_state.get("filter.version"):
        filters += [model.version.in_(st.session_state["filter.version"])]
    if st.session_state.get("filter.id"):
        filters += [model.id == st.session_state["filter.id"]]

    async with session.get_db() as db:
        result = await db.execute(
            select(model).order_by(model.created_at.desc()).filter(*filters).limit(100)
        )

    return pd.DataFrame(
        [
            {
                c: getattr(r, c)
                for c in ["id", "name", "version", "environment", "created_at"]
            }
            for r in result.scalars().all()
        ]
    )


async def get_event_parameters():
    model = models.DB_OBJ_MAP[LogType.EVENT][st.session_state["filter.tier"]]
    async with session.get_db() as db:
        result = await db.execute(
            select(model).where(model.id == st.session_state["event_id"])
        )

    return result.scalars().one().parameters


async def get_event_io(log_type: LogType):
    model = models.DB_OBJ_MAP[log_type][st.session_state["filter.tier"]]
    async with session.get_db() as db:
        result = await db.execute(
            select(model).where(model.parent_id == st.session_state["event_id"])
        )

    return {r.field_name: r.field_value for r in result.scalars().all()}


async def get_event_details() -> dict:
    event_model = models.DB_OBJ_MAP[LogType.EVENT][st.session_state["filter.tier"]]
    runtime_model = models.DB_OBJ_MAP[LogType.RUNTIME][st.session_state["filter.tier"]]

    async with session.get_db() as db:
        event_result = await db.execute(
            select(event_model).where(event_model.id == st.session_state["event_id"])
        )

        runtime_result = await db.execute(
            select(runtime_model).where(
                runtime_model.parent_id == st.session_state["event_id"]
            )
        )

    event = event_result.scalars().one()
    runtime = runtime_result.scalars().one()

    return "\n\n".join(
        [
            f"ID: `{event.id}`",
            f"PARENT ID: `{event.parent_id}`",
            f"START TIME: `{runtime.start_time.isoformat()}`",
            f"END TIME: `{runtime.end_time.isoformat()}`",
            f"ERROR TYPE: `{runtime.error_type}`",
            f"ERROR CONTENT: `{runtime.error_content}`",
        ]
    )

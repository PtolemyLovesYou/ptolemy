"""Service API key management form."""

import streamlit as st
from ptolemy import Workspace, WorkspaceRole, ApiKeyPermission
from ..client import get_client, current_user


@st.fragment
def service_api_key_management_form(
    workspace: Workspace, user_workspace_role: WorkspaceRole
):
    """Service API Key Management"""
    client = get_client()

    disabled = user_workspace_role not in (WorkspaceRole.ADMIN, WorkspaceRole.MANAGER)

    with st.popover("New API Key", use_container_width=True, disabled=disabled):
        with st.form("new_api_key", clear_on_submit=True, border=False):
            new_api_key_name = st.text_input("Name")
            new_api_key_duration = st.slider(
                "Duration (Days)", min_value=1, max_value=365, value=30, step=1
            )

            new_api_key_permission = st.segmented_control(
                "Permission",
                options=list(ApiKeyPermission),
                default=ApiKeyPermission.READ_ONLY,
            )

            submit_new_api_key = st.form_submit_button(label="Create")
            if submit_new_api_key:
                try:
                    api_key = client.create_service_api_key(
                        current_user().id,
                        workspace.id,
                        new_api_key_name,
                        new_api_key_permission,
                        valid_for=new_api_key_duration,
                    )

                    st.markdown("Copy the API key otherwise it will be lost:")
                    st.code(api_key, language=None)
                except ValueError as e:
                    st.error(f"Failed to create API key: {e}")

    keys = client.get_workspace_service_api_keys(workspace.id)

    with st.form("Service API Key Management", clear_on_submit=True, border=False):
        api_keys = st.data_editor(
            [
                {
                    "name": k.name,
                    "preview": f"{k.key_preview}...",
                    "permission": k.permissions,
                    "expires_at": k.expires_at,
                    "delete": False,
                }
                for k in keys
            ],
            use_container_width=True,
            column_config={
                "name": st.column_config.TextColumn(disabled=True),
                "preview": st.column_config.TextColumn(disabled=True),
                "permission": st.column_config.SelectboxColumn(
                    options=list(ApiKeyPermission),
                    disabled=True,
                ),
                "expires_at": st.column_config.DatetimeColumn(
                    disabled=True,
                ),
                "delete": st.column_config.CheckboxColumn(
                    disabled=disabled,
                    required=True,
                ),
            },
        )

        submit_api_keys = st.form_submit_button(label="Save", disabled=disabled)

        if submit_api_keys:
            for key, key_row in zip(keys, api_keys):
                if key_row["delete"]:
                    client.delete_service_api_key(current_user().id, workspace.id, key.id)

            st.rerun(scope="fragment")

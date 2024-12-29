"""Workspace management."""
import streamlit as st
from .models import Workspace, UserRole, WorkspaceRole, ApiKeyPermission, User, ServiceApiKey
from .user import get_users

def add_user_to_workspace_form(workspace: Workspace):
    """Add user to workspace."""
    with st.form("add_user_to_workspace", clear_on_submit=True, border=False):
        valid_users = [
            i for i in get_users() if (
                i.role != UserRole.SYSADMIN
                and i.id not in [usr.id for usr in workspace.users]
                )
            ]

        sk_user = st.selectbox(
            "User",
            options=valid_users,
            format_func=lambda u: u.username,
            index=None,
            placeholder="Select user..."
        )

        sk_role = st.segmented_control(
            "Role",
            options=list(WorkspaceRole),
            default=WorkspaceRole.READER,
        )

        sk_submit = st.form_submit_button(label="Submit")

        if sk_submit:
            success = workspace.add_user(sk_user, sk_role)
            if success:
                st.rerun(scope="fragment")

def create_workspace_form():
    """Create workspace form."""
    with st.form(
        "create_new_wk",
        border=False,
        clear_on_submit=True,
        enter_to_submit=False
        ):
        valid_admins = [i for i in get_users() if i.role != UserRole.SYSADMIN]
        sk_name = st.text_input("Name")
        sk_description = st.text_area("Description")
        sk_admin = st.pills(
            "Admin",
            options=valid_admins,
            format_func=lambda u: u.username,
        )
        sk_submit = st.form_submit_button(label="Create")

        if sk_submit:
            wk = Workspace.create(
                sk_name,
                admin_id=sk_admin.id if sk_admin else None,
                description=sk_description
            )

            if wk:
                st.rerun(scope="fragment")

def workspace_form(workspace: Workspace, user_workspace_role: WorkspaceRole):
    """Workspace form."""
    disabled = user_workspace_role not in (WorkspaceRole.ADMIN, WorkspaceRole.MANAGER)

    with st.form("wk_form", border=False, clear_on_submit=False):
        st.text_input("Name", value=workspace.name, disabled=True)

        st.text_area(
            "Description",
            value=workspace.description,
            key="wk_description",
            disabled=disabled
        )

        submit_wk = st.form_submit_button(
            label="Save",
            disabled=disabled
            )

        if submit_wk:
            st.rerun(scope="fragment")

    with st.popover("Delete Workspace", use_container_width=True, disabled=disabled):
        st.write("Are you sure you want to delete this workspace?")
        delete_wk_button = st.button("Delete", disabled=disabled)
        if delete_wk_button:
            success = workspace.delete()
            if success:
                st.rerun(scope="fragment")

@st.fragment
def wk_user_management_form(workspace: Workspace, user_workspace_role: WorkspaceRole):
    """Workspace user management form"""
    disabled = user_workspace_role not in (WorkspaceRole.ADMIN, WorkspaceRole.MANAGER)
    users = workspace.users

    with st.popover("Add User to Workspace", use_container_width=True, disabled=disabled):
        add_user_to_workspace_form(workspace)

    with st.form("Workspace User Management", clear_on_submit=True, border=False):
        wk_users = st.data_editor(
            [
                {
                    "username": u.username,
                    "role": u.workspace_role(workspace.id),
                    "delete": False
                    } for u in users
                ],
            use_container_width=True,
            column_config={
                "username": st.column_config.TextColumn(disabled=True),
                "role": st.column_config.SelectboxColumn(
                    options=list(WorkspaceRole),
                    disabled=disabled,
                    required=True,
                    ),
                "delete": st.column_config.CheckboxColumn(
                    disabled=disabled,
                    required=True,
                    ),
                },
            )

        submit_wk_users = st.form_submit_button(label="Save", disabled=disabled)

        if submit_wk_users:
            for user, user_row in zip(users, wk_users):
                if user_row["delete"]:
                    workspace.remove_user(user)
                    continue

                if user_row["role"] != user.role:
                    workspace.change_user_role(user, user_row["role"])

            st.rerun(scope="fragment")

@st.fragment
def service_api_key_management_form(workspace: Workspace, user_workspace_role: WorkspaceRole):
    """Service API Key Management"""
    disabled = user_workspace_role not in (WorkspaceRole.ADMIN, WorkspaceRole.MANAGER)

    with st.popover("New API Key", use_container_width=True, disabled=disabled):
        with st.form("new_api_key", clear_on_submit=True, border=False):
            new_api_key_name = st.text_input("Name")
            new_api_key_duration = st.slider(
                "Duration (Days)",
                min_value=1,
                max_value=365,
                value=30,
                step=1
                )

            new_api_key_permission = st.segmented_control(
                "Permission",
                options=list(ApiKeyPermission),
                default=ApiKeyPermission.READ_ONLY
                )

            submit_new_api_key = st.form_submit_button(label="Create")
            if submit_new_api_key:
                api_key = ServiceApiKey.create(
                    workspace,
                    new_api_key_name,
                    new_api_key_permission,
                    duration=new_api_key_duration,
                )

                if api_key is not None:
                    st.markdown("Copy the API key otherwise it will be lost:")
                    st.code(api_key, language=None)

    keys = workspace.api_keys

    with st.form("Service API Key Management", clear_on_submit=True, border=False):
        api_keys = st.data_editor(
            [
                {
                    "name": k.name,
                    "preview": f"{k.key_preview}...",
                    "permission": k.permissions,
                    "expires_at": k.expires_at,
                    "delete": False
                    } for k in keys
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
                    key.delete()

            st.rerun(scope='fragment')

@st.fragment
def wk_management_view():
    """Get workspace management view."""
    wk_selection_col, new_col = st.columns([3, 1])

    with wk_selection_col:
        selected_workspace = st.selectbox(
            "Select workspace",
            options=User.current_user().workspaces,
            format_func=lambda i: i.name,
            placeholder="Select Workspace",
            label_visibility="collapsed",
            index=None,
            )

    with new_col:
        with st.popover(r"\+", use_container_width=True):
            create_workspace_form()

    with st.container(border=True, height=560):
        if selected_workspace is None:
            pass
        else:
            user_workspace_role = User.current_user().workspace_role(selected_workspace.id)

            wk_mgmnt, wk_users, api_keys = st.tabs(
                ["Workspace", "Users", "Service API Keys"]
                )

            with wk_mgmnt:
                workspace_form(
                    selected_workspace,
                    user_workspace_role,
                    )

            with wk_users:
                wk_user_management_form(
                    selected_workspace,
                    user_workspace_role,
                    )

            with api_keys:
                service_api_key_management_form(
                    selected_workspace,
                    user_workspace_role,
                    )

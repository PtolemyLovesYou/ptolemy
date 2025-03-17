"""Test user model."""

from .base import GRAPHQL_MUTATION, IntegrationTestBase

class TestUser(IntegrationTestBase):
    """Test user model."""
    def test_update_own_user(self, admin_jwt: str, admin_user_id: str):
        """Test updating own user."""
        usr = self.graphql(
            admin_jwt,
            GRAPHQL_MUTATION,
            operation_name="UpdateUser",
            variables={"userId": admin_user_id, "displayName": "Updated Admin Display Name"}
        )

        updated_display_name = usr['data']['user']['update']['user']['displayName']

        assert updated_display_name == "Updated Admin Display Name"

    def test_sysadmin_update_user_display_name(self, sysadmin_jwt: str, admin_user_id: str):
        """Test updating user as sysadmin."""
        usr = self.graphql(
            sysadmin_jwt,
            GRAPHQL_MUTATION,
            operation_name="UpdateUser",
            variables={"userId": admin_user_id, "displayName": "Name Updated By Sysadmin"}
        )

        assert not usr['data']['user']['update']['success']

    def test_sysadmin_update_user_status(self, sysadmin_jwt: str, admin_user_id: str):
        """Test updating user email as sysadmin."""
        usr = self.graphql(
            sysadmin_jwt,
            GRAPHQL_MUTATION,
            operation_name="UpdateUser",
            variables={"userId": admin_user_id, "status": "SUSPENDED"}
        )

        assert usr['data']['user']['update']['user']['status'] == "SUSPENDED"
        assert usr['data']['user']['update']['success']

        # undo
        usr = self.graphql(
            sysadmin_jwt,
            GRAPHQL_MUTATION,
            operation_name="UpdateUser",
            variables={"userId": admin_user_id, "status": "ACTIVE"}
        )

        assert usr['data']['user']['update']['user']['status'] == "ACTIVE"
        assert usr['data']['user']['update']['success']

"""Test sysadmin permissions."""

from .base import IntegrationTestBase, GRAPHQL_QUERY


class TestSysadminPermissions(IntegrationTestBase):
    """Test sysadmin permissions."""

    def test_is_sysadmin(self, sysadmin_jwt: str):
        """Test is sysadmin."""
        resp = self.graphql(
            sysadmin_jwt,
            query=GRAPHQL_QUERY,
            operation_name="Me",
        )

        assert resp["data"]["me"]["isSysadmin"]

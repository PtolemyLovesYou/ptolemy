"""Test sysadmin permissions."""

import requests
from .base import IntegrationTestBase, GRAPHQL_QUERY


class TestSysadminPermissions(IntegrationTestBase):
    """Test sysadmin permissions."""

    def test_is_sysadmin(self, sysadmin_jwt: str):
        """Test is sysadmin."""
        resp = requests.post(
            "http://localhost:8000/graphql",
            json={
                "query": GRAPHQL_QUERY,
                "operationName": "Me",
            },
            headers={
                "Authorization": f"Bearer {sysadmin_jwt}",
                "Content-Type": "application/json",
            },
            timeout=30,
        )

        assert resp.status_code == 200, f"Error: {resp.text}"

        assert resp.json()["data"]["me"]["isSysadmin"]

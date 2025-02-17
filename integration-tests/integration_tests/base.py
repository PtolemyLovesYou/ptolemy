"""Integration Test Base."""

import os
import time
from typing import Generator
import requests
import pytest
from ptolemy import get_client, Ptolemy # pylint: disable=no-name-in-module

SYSADMIN_USERNAME = os.getenv("SYSADMIN_USERNAME", "admin")
SYSADMIN_PASSWORD = os.getenv("SYSADMIN_PASSWORD", "admin")
GRAPHQL_PATH = os.getenv("GRAPHQL_PATH", "../ptolemy/graphql")

USER_USERNAME = f"test_user_{int(time.time())}"
USER_PASSWORD = USER_USERNAME

with open(os.path.join(GRAPHQL_PATH, "query.gql"), encoding="utf-8") as f:
    GRAPHQL_QUERY = f.read()

with open(os.path.join(GRAPHQL_PATH, "mutation.gql"), encoding="utf-8") as f:
    GRAPHQL_MUTATION = f.read()

BASE_URL = "http://localhost:8000/"

class IntegrationTestBase:
    """Base Integration Test."""

    @pytest.fixture(scope="class")
    def sysadmin_jwt(self) -> Generator[str, None, None]:
        """Superadmin JWT."""
        resp = requests.post(
            os.path.join(BASE_URL, "auth"),
            json={
                "username": SYSADMIN_USERNAME,
                "password": SYSADMIN_PASSWORD,
            },
            timeout=10,
        )

        assert resp.status_code == 200, f"Error: {resp.text}"

        yield resp.json()["token"]

    @pytest.fixture(scope="class")
    def workspace_name(self) -> Generator[str, None, None]:
        """Workspace Name."""
        yield f"test_workspace_{int(time.time())}"

    @pytest.fixture(scope="class")
    def admin_user_id(self, sysadmin_jwt: str, admin_username: str) -> Generator[str, None, None]:
        """Create admin user."""
        resp = requests.post(
            os.path.join(BASE_URL, "graphql"),
            json={
                "query": GRAPHQL_MUTATION,
                "operationName": "CreateUser",
                "variables": {
                    "username": admin_username,
                    "password": admin_username,
                    "displayName": "Test User (integration tests)",
                    "isAdmin": True,
                },
            },
            timeout=30,
            headers={
                "Authorization": f"Bearer {sysadmin_jwt}",
                "Content-Type": "application/json",
            },
        )

        assert resp.status_code == 200, f"Error: {resp.text}"

        user_id = resp.json()["data"]["user"]["create"]["user"]["id"]

        yield user_id

        resp = requests.post(
            os.path.join(BASE_URL, "graphql"),
            json={
                "query": GRAPHQL_MUTATION,
                "operationName": "DeleteUser",
                "variables": {"userId": user_id},
            },
            timeout=30,
            headers={
                "Authorization": f"Bearer {sysadmin_jwt}",
                "Content-Type": "application/json",
            },
        )

        assert resp.status_code == 200, f"Error: {resp.text}"

        delete_resp = requests.post(
            os.path.join(BASE_URL, "graphql"),
            json={
                "query": GRAPHQL_MUTATION,
                "operationName": "DeleteUser",
                "variables": {"userId": user_id},
            },
            timeout=30,
            headers={
                "Authorization": f"Bearer {sysadmin_jwt}",
                "Content-Type": "application/json",
            },
        )

        assert delete_resp.status_code == 200, f"Error: {delete_resp.text}"

    @pytest.fixture(scope="class")
    def admin_jwt(self, admin_user_id: str, admin_username: str) -> Generator[str, None, None]:
        """Admin JWT."""
        resp = requests.post(
            os.path.join(BASE_URL, "auth"),
            json={
                "username": admin_username,
                "password": admin_username,
            },
            timeout=10,
        )

        assert resp.status_code == 200, f"Error for {admin_user_id}: {resp.text}"

        yield resp.json()["token"]

    @pytest.fixture(scope="class")
    def admin_username(self) -> Generator[str, None, None]:
        """Username."""
        yield f"test_user_{int(time.time())}"

    @pytest.fixture(scope="class")
    def admin_api_key(self, admin_jwt: str) -> Generator[str, None, None]:
        """Admin API Key."""
        resp = requests.post(
            os.path.join(BASE_URL, "graphql"),
            json={
                "query": GRAPHQL_MUTATION,
                "operationName": "CreateUserApiKey",
                "variables": {
                    "name": f"test_{int(time.time())}",
                    "durationDays": 1,
                },
            },
            timeout=10,
            headers={
                "Authorization": f"Bearer {admin_jwt}",
                "Content-Type": "application/json",
            },
        )

        assert resp.status_code == 200, f"Error: {resp.text}"

        api_key = resp.json()["data"]["user"]["createUserApiKey"]["apiKey"]["apiKey"]
        yield api_key

    @pytest.fixture(scope="class")
    def workspace_id(
        self, admin_api_key: str, admin_user_id: str, workspace_name: str
    ) -> Generator[str, None, None]:
        """Workspace Name."""
        resp = requests.post(
            os.path.join(BASE_URL, "graphql"),
            json={
                "query": GRAPHQL_MUTATION,
                "operationName": "CreateWorkspace",
                "variables": {
                    "name": workspace_name,
                    "description": "Test Workspace (integration tests)",
                    "adminUserId": admin_user_id,
                },
            },
            timeout=10,
            headers={"X-Api-Key": admin_api_key, "Content-Type": "application/json"},
        )

        assert resp.status_code == 200, f"Error: {resp.text}"
        try:
            workspace_id = resp.json()["data"]["workspace"]["create"]["workspace"]["id"]
        except TypeError as exc:
            raise TypeError(
                f"Failed to get workspace id from response: {resp.json()}"
            ) from exc

        yield workspace_id

        resp = requests.post(
            os.path.join(BASE_URL, "graphql"),
            json={
                "query": GRAPHQL_MUTATION,
                "operationName": "DeleteWorkspace",
                "variables": {"workspaceId": workspace_id},
            },
            timeout=10,
            headers={"X-Api-Key": admin_api_key, "Content-Type": "application/json"},
        )

        assert resp.status_code == 200, f"Error: {resp.text}"

    def _service_api_key(
        self, admin_api_key: str, workspace_id: str, permission: str
    ) -> Generator[str, None, None]:
        """Service API Key."""
        resp = requests.post(
            os.path.join(BASE_URL, "graphql"),
            json={
                "query": GRAPHQL_MUTATION,
                "operationName": "CreateServiceApiKey",
                "variables": {
                    "name": f"test_{int(time.time())}",
                    "durationDays": 1,
                    "permission": permission,
                    "workspaceId": workspace_id,
                },
            },
            timeout=10,
            headers={"X-Api-Key": admin_api_key, "Content-Type": "application/json"},
        )

        assert resp.status_code == 200, f"Error: {resp.text}"

        service_api_key = resp.json()["data"]["workspace"]["createServiceApiKey"][
            "apiKey"
        ]["apiKey"]
        yield service_api_key

    @pytest.fixture
    def ro_service_api_key(
        self, admin_api_key: str, workspace_id: str
    ) -> Generator[str, None, None]:
        """Read Only Service API Key."""
        yield from self._service_api_key(admin_api_key, workspace_id, "READ_ONLY")

    @pytest.fixture
    def rw_service_api_key(
        self, admin_api_key: str, workspace_id: str
    ) -> Generator[str, None, None]:
        """Read Write Service API Key."""
        yield from self._service_api_key(admin_api_key, workspace_id, "READ_WRITE")

    @pytest.fixture
    def wo_service_api_key(
        self, admin_api_key: str, workspace_id: str
    ) -> Generator[str, None, None]:
        """Write Only Service API Key."""
        yield from self._service_api_key(admin_api_key, workspace_id, "WRITE_ONLY")

    @pytest.fixture
    def client(self, rw_service_api_key: str, workspace_name: str) -> Ptolemy:
        """Ptolemy Client."""
        return get_client(
            base_url=BASE_URL,
            api_key=rw_service_api_key,
            workspace_name=workspace_name,
            autoflush=False,
            batch_size=256,
        )

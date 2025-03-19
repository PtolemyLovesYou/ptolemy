"""Test sysadmin permissions."""

import time
import pandas as pd
import pytest
from ptolemy_client import Ptolemy, get_client  # pylint: disable=no-name-in-module
from .base import IntegrationTestBase, BASE_URL


class TestSql(IntegrationTestBase):
    """Test sql functionality."""

    def test_user_client(self, admin_api_key: str):
        """Test user client."""
        client = get_client(
            base_url=BASE_URL,
            api_key=admin_api_key,
        )

        client.sql("SELECT 1;")

    def test_user_client_disable_writing(self, admin_api_key: str):
        """Test user client."""
        client = get_client(
            base_url=BASE_URL,
            api_key=admin_api_key,
            autoflush=False,
        )

        with pytest.raises(ValueError):
            client.trace("asdf")

    def test_write_logs(self, client: Ptolemy):
        """Test workspaces"""

        for _ in range(10):
            sys = client.trace("test_trace", version="1.2.3", environment="dev")
            with sys:
                sys.inputs(
                    foo={"bar": "baz"},
                    baz=1,
                    qux=True,
                    test_str="this is a string",
                    test_float=0.93,
                )

        client.flush()

        # Wait for all events to be processed
        time.sleep(2)
        try:
            result = pd.concat(client.sql("SELECT id FROM ptolemy.system_event"))

            assert len(result) > 0
        except ValueError as exc:
            raise ValueError("No system events returned") from exc

    def test_get_events(
        self, client: Ptolemy, rw_service_api_key: str, workspace_id: str
    ):
        """Test workspaces"""

        query = """
        query TestBasicSystemEvents {
          systemEvents {
            id
            name
            version
            environment
            runtime {
              startTime
              endTime
              errorType
              errorContent
            }
          }
        }
        """

        result = self.graphql_api_key(
            rw_service_api_key, query, operation_name="TestBasicSystemEvents"
        )

        assert result["data"]["systemEvents"]

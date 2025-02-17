"""Test sysadmin permissions."""

import time
import pandas as pd
from ptolemy import Ptolemy # pylint: disable=no-name-in-module
from .base import IntegrationTestBase

class TestSql(IntegrationTestBase):
    """Test sql functionality."""
    def test_write_logs(self, client: Ptolemy):
        """Test workspaces"""

        for _ in range(10):
            sys = client.trace("test_trace", version='1.2.3', environment='dev')
            with sys:
                sys.inputs(
                    foo={"bar": "baz"},
                    baz=1,
                    qux=True,
                    test_str="this is a string",
                    test_float=0.93
                    )

        client.flush()

        # Wait for all events to be processed
        time.sleep(2)
        try:
            result = pd.concat(client.sql("SELECT id FROM ptolemy.system_event"))

            assert len(result) > 0
        except ValueError as exc:
            raise ValueError("No system events returned") from exc

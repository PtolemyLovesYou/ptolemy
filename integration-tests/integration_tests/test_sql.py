"""Test sysadmin permissions."""

import pytest
import pandas as pd
from ptolemy_client import Ptolemy  # pylint: disable=no-name-in-module
from .base import IntegrationTestBase


class TestSql(IntegrationTestBase):
    """Test sql functionality."""

    def test_naive_select(self, client: Ptolemy):
        """Test select"""
        result = pd.concat(client.sql("SELECT 1;"))

        assert result.shape == (1, 1)
        assert result.iloc[0, 0] == 1

    def test_error(self, client: Ptolemy):
        """Test sql query with syntactical errors."""
        with pytest.raises(ValueError):
            client.sql("select * froma asdf")

    def test_workspaces(self, client: Ptolemy, workspace_id: str):
        """Test workspaces"""
        result = pd.concat(client.sql("SELECT id FROM ptolemy.workspace"))

        assert len(result) == 1
        assert result.iloc[0, 0] == workspace_id

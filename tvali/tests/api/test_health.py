"""Test create."""

from fastapi.testclient import TestClient
from tvali.api.main import app

client = TestClient(app)


def test_health():
    """Test health endpoint."""
    response = client.get("/health")

    assert response.status_code == 200

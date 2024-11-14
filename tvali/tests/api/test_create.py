"""Test create methods."""
from uuid import uuid4
from datetime import datetime
import pytest
from fastapi.testclient import TestClient
from tvali.api.main import app
from tvali.utils import Tier, LogType

client = TestClient(app)

tier_ids = {tier.value: uuid4().hex for tier in Tier}

def get_event_data(tier: Tier):
    """Get event data."""
    return {
        "id": tier_ids[tier],
        "name": "test",
        "parameters": {"foo": "bar"},
        "environment": "DEV",
        "version": "0.0.1test",
    }

def get_runtime_data(tier: Tier):
    """Get runtime data."""
    return {
        "id": uuid4().hex,
        f"{tier}_event_id": tier_ids[tier],
        "start_time": datetime.now().isoformat(),
        "end_time": datetime.now().isoformat(),
    }

def get_io_data(tier: Tier):
    """Get IO data."""
    return {
        "id": uuid4().hex,
        f"{tier}_event_id": tier_ids[tier],
        "field_name": "foo",
        "field_value": "bar",
    }

@pytest.mark.parametrize("log_type", [*LogType])
@pytest.mark.parametrize("tier", [*Tier])
@pytest.mark.dependency(name="test_create")
def test_create(tier: Tier, log_type: LogType):
    """Test create log."""
    if log_type == LogType.EVENT:
        event_data = get_event_data(tier)
        if tier.parent is not None:
            event_data[f"{tier.parent}_event_id"] = tier_ids[tier.parent]

    elif log_type == LogType.RUNTIME:
        event_data = get_runtime_data(tier)
    else:
        event_data = get_io_data(tier)

    response = client.post(
        f"/v1/log/{tier.value}/{log_type.value}",
        json=[event_data],
        )

    assert response.status_code == 200
    assert response.json()[0]["id"] == event_data["id"]

@pytest.mark.parametrize("tier", [*Tier])
@pytest.mark.dependency(name="test_get_log", depends=["test_create"])
def test_get_log(tier: Tier):
    """Test get log."""
    response = client.get(f"/v1/log/{tier.value}/event/?id={tier_ids[tier]}")

    assert response.status_code == 200, response.text
    assert response.json()[0]["id"] == tier_ids[tier]

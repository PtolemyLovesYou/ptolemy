"""Test create methods."""
from uuid import uuid4
from datetime import datetime
import pytest
from fastapi.testclient import TestClient
from tvali.api.main import app
from tvali.utils import Tier, LogType

client = TestClient(app)

tier_ids = {tier.value: uuid4().hex for tier in Tier}

@pytest.mark.parametrize("tier", [Tier.SYSTEM, Tier.SUBSYSTEM, Tier.COMPONENT, Tier.SUBCOMPONENT])
@pytest.mark.dependency(name="test_create_event")
def test_create_event(tier: Tier):
    """Test create log."""
    event_data = {
        "id": tier_ids[tier],
        "name": "test",
        "parameters": {"foo": "bar"},
        "environment": "DEV",
        "version": "0.0.1test",
    }

    if tier.parent is not None:
        event_data[f"{tier.parent}_event_id"] = tier_ids[tier.parent]

    response = client.post(
        f"/v1/log/{tier.value}/event",
        json=[event_data],
        )

    assert response.status_code == 200
    assert response.json()[0]["id"] == tier_ids[tier]

@pytest.mark.parametrize("tier", [Tier.SYSTEM, Tier.SUBSYSTEM, Tier.COMPONENT, Tier.SUBCOMPONENT])
@pytest.mark.dependency(name="test_create_runtime", depends=["test_create_event"])
def test_create_runtime(tier: Tier):
    runtime_data = {
        "id": uuid4().hex,
        f"{tier}_event_id": tier_ids[tier],
        "start_time": datetime.now().isoformat(),
        "end_time": datetime.now().isoformat(),
        "error_type": "test",
        "error_content": "test",
    }

    response = client.post(
        f"/v1/log/{tier.value}/runtime",
        json=[runtime_data],
        )

    assert response.status_code == 200
    assert response.json()[0]["id"] == runtime_data["id"]

@pytest.mark.parametrize("tier", [Tier.SYSTEM, Tier.SUBSYSTEM, Tier.COMPONENT, Tier.SUBCOMPONENT])
@pytest.mark.parametrize("log_type", [LogType.INPUT, LogType.OUTPUT, LogType.FEEDBACK, LogType.METADATA])
@pytest.mark.dependency(name="test_create_io", depends=["test_create_event"])
def test_create_io(tier: Tier, log_type: LogType):
    io_data = {
        "id": uuid4().hex,
        f"{tier}_event_id": tier_ids[tier],
        "field_name": "foo",
        "field_value": "bar",
    }

    url = f"/v1/log/{tier.value}/{log_type.value}"
    print(url)

    response = client.post(
        url,
        json=[io_data],
        )

    assert response.status_code == 200, response.text
    assert response.json()[0]["id"] == io_data["id"]

@pytest.mark.parametrize("tier", [Tier.SYSTEM, Tier.SUBSYSTEM, Tier.COMPONENT, Tier.SUBCOMPONENT])
@pytest.mark.dependency(name="test_get_event", depends=["test_create_event", "test_create_runtime", "test_create_io"])
def test_get_log(tier: Tier):
    response = client.get(f"/v1/log/{tier.value}/event/?id={tier_ids[tier]}")

    assert response.status_code == 200, response.text
    assert response.json()[0]["id"] == tier_ids[tier]

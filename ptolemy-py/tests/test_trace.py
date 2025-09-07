"""Test Trace functionality."""

from uuid import uuid4
import pytest
from ptolemy_client.client.mock import MockPtolemy
from ptolemy_client.tier import Tier

@pytest.fixture
def client() -> MockPtolemy:
    return MockPtolemy()

def test_trace_subject_id(client: MockPtolemy):
    trace = client.trace(
        uuid4(),  # subject_id
        "my_system_trace",
        parameters={"foo": "bar"},
        version="0.0.1",
        environment="DEV",
    )

    assert trace.subject_id == trace.parent_id
    assert trace.tier == Tier.SYSTEM

def test_trace_child(client: MockPtolemy):
    trace = client.trace(
        uuid4(),  # subject_id
        "my_system_trace",
        parameters={"foo": "bar"},
        version="0.0.1",
        environment="DEV",
    )

    child = trace.child(
        "child", parameters={"foo": "bar"}, version="0.0.1", environment="DEV"
    )

    assert child.subject_id == trace.subject_id
    assert child.parent_id == trace.id_
    assert child.tier == Tier.SUBSYSTEM

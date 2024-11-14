"""Test API client."""

import os
import pytest
from tvali.client.clients.api.client import Tvali
from tvali.client import TvaliClient

event_dict = {
    "name": "foo",
    "parameters": {"foo": "bar"},
    "version": "0.0.1",
    "environment": "DEV",
}


@pytest.mark.asyncio
async def test_api_client():
    """Test API client."""
    base_url = os.getenv("TVALI_API_URL")
    if base_url is None:
        raise ValueError("TVALI_API_URL is not set")

    client: TvaliClient = Tvali(base_url=base_url)

    s_log = client.trace(
        "foo",
        parameters={"foo": "bar"},
        version="0.0.1",
        environment="DEV",
    )

    async with s_log.observe(time=True):
        ss_log = s_log.spawn(**event_dict)
        async with ss_log.observe(time=True):
            c_log = ss_log.spawn(**event_dict)
            async with c_log.observe(time=True):
                sc_log = c_log.spawn(**event_dict)
                async with sc_log.observe(time=True):
                    await sc_log.log(
                        inputs={"foo": "bar"},
                        outputs={"foo": "bar"},
                        feedback={"foo": "bar"},
                        metadata={"foo": "bar"},
                    )

    await sc_log.delete()
    await c_log.delete()
    await ss_log.delete()
    await s_log.delete()

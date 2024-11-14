"""Rest API Log."""

from typing import List, ClassVar
import asyncio
import aiohttp
from .config import TvaliConfig
from ..log.core import Log


class APILog(Log):
    """API Log."""

    TRANSPORT_CONFIG: ClassVar[TvaliConfig]

    async def _push_io(
        self, session: aiohttp.ClientSession, url: str, data: List[dict]
    ) -> None:
        if data is not None:
            async with session.post(url, json=data) as response:
                await response.json()

    async def _push_event(self, session: aiohttp.ClientSession) -> None:
        # event data
        event = [self.event_dict()]

        async with session.post(
            f"/v1/log/{self.TIER.value}/event", json=event
        ) as response:
            print(await response.json())

    async def _push_runtime(self, session: aiohttp.ClientSession) -> None:
        # runtime data
        runtime = [self.runtime_dict()]

        async with session.post(
            f"/v1/log/{self.TIER.value}/runtime", json=runtime
        ) as response:
            await response.json()

    async def delete(self) -> None:
        async with aiohttp.ClientSession(
            base_url=self.TRANSPORT_CONFIG.base_url
        ) as session:
            async with session.delete(
                f"/v1/log/{self.TIER.value}/event/{self.id.model_dump()}"  # pylint: disable=no-member
            ) as response:
                await response.json()

    async def push_on_beginning(self) -> None:
        async with aiohttp.ClientSession(
            base_url=self.TRANSPORT_CONFIG.base_url
        ) as session:
            await self._push_event(session)

    async def push_on_ending(self) -> None:
        async with aiohttp.ClientSession(
            base_url=self.TRANSPORT_CONFIG.base_url
        ) as session:
            await asyncio.gather(
                self._push_runtime(session),
                self._push_io(
                    session, f"/v1/log/{self.TIER.value}/input", self.inputs_dicts()
                ),
                self._push_io(
                    session, f"/v1/log/{self.TIER.value}/output", self.outputs_dicts()
                ),
                self._push_io(
                    session,
                    f"/v1/log/{self.TIER.value}/feedback",
                    self.feedback_dicts(),
                ),
                self._push_io(
                    session,
                    f"/v1/log/{self.TIER.value}/metadata",
                    self.metadata_dicts(),
                ),
            )

"""Log endpoints."""
from typing import Callable
from pydantic import BaseModel
from fastapi import APIRouter
from ..schemas.log import LogSchema, Create, Record
from ....utils.enums import Tier, LogType


class LogEndpointFactory(BaseModel):
    """Log endpoint factory."""
    tier: Tier
    log_type: LogType

    def create_endpoint(self) -> Callable:
        """Generate create endpoint."""
        async def create(data: LogSchema[self.tier, self.log_type, Create]) -> dict:
            return data

        return create

    def get_endpoint(self) -> Callable:
        """Generate get endpoint."""
        async def get(id_: str) -> LogSchema[self.tier, self.log_type, Record]: # pylint: disable=unused-argument
            return {}

        return get

    def delete_endpoint(self) -> Callable:
        """Generate delete endpoint."""
        async def delete(id_: str) -> dict: # pylint: disable=unused-argument
            return {"status": "success"}

        return delete

    def __call__(self) -> APIRouter:
        router_ = APIRouter(
            prefix=f"/{self.tier}/{self.log_type}",
            tags=[f"{self.tier}_{self.log_type}"],
        )

        router_.add_api_route("/", self.create_endpoint(), methods=["POST"])
        router_.add_api_route("/{id_}", self.get_endpoint(), methods=["GET"])
        router_.add_api_route("/{id_}", self.delete_endpoint(), methods=["DELETE"])

        return router_

def create_router() -> APIRouter:
    """
    Create a router for logs with all endpoints.

    This function returns an APIRouter that contains endpoints for all combinations of
    tier and log type. The endpoints are as follows:

    - POST /{tier}/{log_type}/: Create a new log entry.
    - GET /{tier}/{log_type}/{id_}: Retrieve a log entry by id.
    - DELETE /{tier}/{log_type}/{id_}: Delete a log entry by id.

    The router is tagged with "log".
    """
    log_router = APIRouter(
        prefix="/log",
        tags=["log"],
    )

    for tier in Tier:
        for log_type in LogType:
            log_router.include_router(LogEndpointFactory(tier=tier, log_type=log_type)())

    return log_router

router = create_router()

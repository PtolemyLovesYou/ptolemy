"""Log endpoints."""

from pydantic import BaseModel
from fastapi import APIRouter
from tvali_utils.enums import Tier, LogType
from ..crud.log import LogCRUDFactory


class LogEndpointFactory(BaseModel):
    """Log endpoint factory."""

    tier: Tier
    log_type: LogType

    def __call__(self) -> APIRouter:
        router_ = APIRouter(
            prefix=f"/{self.tier}/{self.log_type}",
            tags=[f"{self.tier}_{self.log_type}"],
        )

        crud_factory = LogCRUDFactory(tier=self.tier, log_type=self.log_type)

        router_.add_api_route("/", crud_factory.create_function(), methods=["POST"])
        router_.add_api_route("/", crud_factory.get_function(), methods=["GET"])
        router_.add_api_route(
            "/{id_}", crud_factory.delete_function(), methods=["DELETE"]
        )

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
            log_router.include_router(
                LogEndpointFactory(tier=tier, log_type=log_type)()
            )

    return log_router


router = create_router()

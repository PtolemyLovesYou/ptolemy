"""Event endpoints."""

from uuid import UUID
from fastapi import APIRouter

from ..schemas.event import (
    CreateSchemaMixin,
    RecordSchemaMixin,
    event_record_type_mixin,
    dependent_mixin
    )
from ..crud.event import get_event, create_event, delete_event
from ....db import models
from ....utils.enums import Tier, EventRecordType

router = APIRouter(
    prefix="/record",
    tags=["record"],
)

DB_MODEL_MAP = {
    t: {
        et: getattr(models, f"{t.capitalize()}{et.capitalize()}")
        for et in EventRecordType
    }
    for t in Tier
}


def endpoint_factory(tier: Tier, event_type: EventRecordType) -> APIRouter:
    """
    Create an APIRouter with endpoints for the given tier and event type.

    Args:
        tier: The tier.
        event_type: The event type.

    Returns:
        An APIRouter with endpoints for the given tier and event type.
    """
    db_model = DB_MODEL_MAP[tier][event_type]

    class ModelSchemaBase(
        dependent_mixin(tier, event_type), event_record_type_mixin(event_type)
    ):  # pylint: disable=missing-class-docstring,too-few-public-methods
        pass

    ModelSchemaBase.__name__ = f"{tier.capitalize()}{event_type.capitalize()}Base"
    ModelSchemaBase.__doc__ = (
        f"Base class for {tier.capitalize}{event_type.capitalize()}."
    )

    class ModelSchemaCreate(
        ModelSchemaBase, CreateSchemaMixin
    ):  # pylint: disable=missing-class-docstring,too-few-public-methods
        pass

    ModelSchemaCreate.__name__ = f"{tier.capitalize()}{event_type.capitalize()}Create"
    ModelSchemaCreate.__doc__ = (
        f"Create class for {tier.capitalize}{event_type.capitalize()}."
    )

    class ModelSchemaRecord(
        ModelSchemaBase, RecordSchemaMixin
    ):  # pylint: disable=missing-class-docstring,too-few-public-methods
        pass

    ModelSchemaRecord.__name__ = f"{tier.capitalize()}{event_type.capitalize()}"
    ModelSchemaRecord.__doc__ = (
        f"Record class for {tier.capitalize()}{event_type.capitalize()}."
    )

    fct_router = APIRouter(
        prefix=f"/{tier}/{event_type}",
        tags=[tier, event_type],
    )

    async def get_system_event(id_: str) -> ModelSchemaRecord:
        """Get system event."""
        return get_event(db_model, ModelSchemaRecord, id_)

    async def create_system_event(event: ModelSchemaCreate) -> dict[str, UUID]:
        """Create event."""
        return create_event(event, db_model)

    async def delete_system_event(id_: str) -> dict[str, str]:
        """Delete event."""
        return delete_event(db_model, id_)

    fct_router.add_api_route("/{id}", get_system_event, methods=["GET"])
    fct_router.add_api_route("/", create_system_event, methods=["POST"])
    fct_router.add_api_route("/{id}", delete_system_event, methods=["DELETE"])

    return fct_router


for t in Tier:
    for et in EventRecordType:
        router.include_router(endpoint_factory(t, et))

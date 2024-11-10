"""Event endpoints."""

from uuid import UUID
from fastapi import APIRouter

from ..schemas.event import Tier, EventRecordType, CreateSchemaMixin, RecordSchemaMixin
from ..crud.event import get_event, create_event, delete_event
from ....db import models

router = APIRouter(
    prefix="/event",
    tags=["event"],
)

DB_MODEL_MAP = {
    Tier.SYSTEM: {
        EventRecordType.EVENT: models.SystemEvent,
        EventRecordType.RUNTIME: models.SystemRuntime,
        EventRecordType.INPUT: models.SystemInput,
        EventRecordType.OUTPUT: models.SystemOutput,
        EventRecordType.FEEDBACK: models.SystemFeedback,
        EventRecordType.METADATA: models.SystemMetadata,
    },
    Tier.SUBSYSTEM: {
        EventRecordType.EVENT: models.SubsystemEvent,
        EventRecordType.RUNTIME: models.SubsystemRuntime,
        EventRecordType.INPUT: models.SubsystemInput,
        EventRecordType.OUTPUT: models.SubsystemOutput,
        EventRecordType.FEEDBACK: models.SubsystemFeedback,
        EventRecordType.METADATA: models.SubsystemMetadata,
    },
    Tier.COMPONENT: {
        EventRecordType.EVENT: models.ComponentEvent,
        EventRecordType.RUNTIME: models.ComponentRuntime,
        EventRecordType.INPUT: models.ComponentInput,
        EventRecordType.OUTPUT: models.ComponentOutput,
        EventRecordType.FEEDBACK: models.ComponentFeedback,
        EventRecordType.METADATA: models.ComponentMetadata,
    },
    Tier.SUBCOMPONENT: {
        EventRecordType.EVENT: models.SubcomponentEvent,
        EventRecordType.RUNTIME: models.SubcomponentRuntime,
        EventRecordType.INPUT: models.SubcomponentInput,
        EventRecordType.OUTPUT: models.SubcomponentOutput,
        EventRecordType.FEEDBACK: models.SubcomponentFeedback,
        EventRecordType.METADATA: models.SubcomponentMetadata,
    },
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

    class ModelSchemaBase(tier.dependent_mixin(event_type), event_type.mixin()):
        """Base class for model schema."""

    class ModelSchemaCreate(ModelSchemaBase, CreateSchemaMixin):
        """Create class for model schema."""

    class ModelSchemaRecord(ModelSchemaBase, RecordSchemaMixin):
        """Record class for model schema."""

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

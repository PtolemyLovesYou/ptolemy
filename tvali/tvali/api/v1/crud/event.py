"""CRUD operations for events with proper error handling and connection management."""

from typing import TypeVar, Type
from uuid import UUID
import logging
from sqlalchemy.exc import SQLAlchemyError
from fastapi import HTTPException, status
from ....db import models, session
from ..schemas.core import RecordSchemaMixin, CreateSchemaMixin

# Setup logging
logger = logging.getLogger(__name__)

# Type variables for better type hinting
EventModel = TypeVar("EventModel", bound=models.EventTable)
RecordType = TypeVar(
    "RecordType", bound=RecordSchemaMixin
)  # pylint: disable=invalid-name


class EventNotFoundError(Exception):
    """Raised when an event is not found in the database."""


def validate_uuid(idx: str) -> UUID:
    """
    Validate and convert string UUID to UUID object.

    Args:
        idx (str): UUID string to validate

    Returns:
        UUID: Validated UUID object

    Raises:
        HTTPException: If UUID is invalid
    """
    try:
        return UUID(idx)
    except ValueError as exc:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST, detail="Invalid UUID format"
        ) from exc


def get_event(
    db_class: Type[EventModel], return_class: Type[RecordType], idx: str
) -> RecordType:
    """
    Get event by ID with error handling.

    Args:
        db_class: Database model class
        return_class: Pydantic model class for response
        idx: Event ID

    Returns:
        RecordType: Event data

    Raises:
        HTTPException: If event not found or database error occurs
    """
    validated_id = validate_uuid(idx)

    with session.get_db() as db:
        try:
            event = db.query(db_class).filter(db_class.id == validated_id).first()
            if not event:
                raise EventNotFoundError(f"Event with id {idx} not found")

            return return_class.model_validate(event.__dict__)

        except EventNotFoundError as e:
            logger.warning("Event not found: %s", e)
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND, detail=str(e)
            ) from e
        except SQLAlchemyError as e:
            logger.error("Database error in get_event: %s", e)
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail="Internal server error while retrieving event",
            ) from e


def create_event(
    data: CreateSchemaMixin, db_class: Type[EventModel]
) -> dict[str, UUID]:
    """
    Create new event with error handling.

    Args:
        data: Validated input data
        db_class: Database model class

    Returns:
        dict: Created event ID

    Raises:
        HTTPException: If database error occurs
    """
    with session.get_db() as db:
        try:
            obj = db_class(**data.model_dump(exclude_unset=True))
            db.add(obj)
            db.commit()
            db.refresh(obj)

            logger.info("Created new event with id: %s", obj.id)
            return {"id": obj.id}

        except SQLAlchemyError as e:
            db.rollback()
            logger.error("Database error in create_event: %s", e)
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail="Internal server error while creating event",
            ) from e


def delete_event(db_class: Type[EventModel], idx: str) -> dict[str, str]:
    """
    Delete event by ID with error handling.

    Args:
        db_class: Database model class
        idx: Event ID

    Returns:
        dict: Success status

    Raises:
        HTTPException: If event not found or database error occurs
    """
    validated_id = validate_uuid(idx)

    with session.get_db() as db:
        try:
            result = db.query(db_class).filter(db_class.id == validated_id).delete()
            if not result:
                raise EventNotFoundError(f"Event with id {idx} not found")

            db.commit()
            logger.info("Deleted event with id: %s", idx)
            return {"status": "success"}

        except EventNotFoundError as e:
            logger.warning("Event not found for deletion: %s", e)
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND, detail=str(e)
            ) from e
        except SQLAlchemyError as e:
            db.rollback()
            logger.error("Database error in delete_event: %s", e)
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail="Internal server error while deleting event",
            ) from e

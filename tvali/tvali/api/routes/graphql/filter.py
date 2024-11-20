"""Input fields."""
from typing import Optional
import strawberry
from strawberry.scalars import ID

@strawberry.interface
class QueryFilter:
    """Query filter."""
    id: Optional[ID] = strawberry.UNSET

@strawberry.input
class EventFilter(QueryFilter):
    """Event filter."""
    name: Optional[str] = strawberry.UNSET
    environment: Optional[str] = strawberry.UNSET
    version: Optional[str] = strawberry.UNSET

@strawberry.input
class IOFilter(QueryFilter):
    """IO Filter."""
    field_name: Optional[str] = strawberry.UNSET

@strawberry.input
class MetadataFilter(QueryFilter):
    """Metadata filter."""
    field_name: Optional[str] = strawberry.UNSET
    field_value: Optional[str] = strawberry.UNSET

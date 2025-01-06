"""GraphQL Response model."""

from typing import Callable
import functools
from .query import GQLQuery
from .mutation import GQLMutation

class GQLResponseException(Exception):
    """GQL Response Exception."""

def uses_gql(func: Callable) -> Callable:
    """Uses GQL."""
    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        try:
            return func(*args, **kwargs)
        except AttributeError as e:
            # This is temporary
            raise GQLResponseException(
                f"The fields you want are not in your GraphQL object!! {e}"
                ) from e

    return wrapper

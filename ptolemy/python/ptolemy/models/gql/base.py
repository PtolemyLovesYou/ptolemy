"""GraphQL Response model."""

from typing import Optional, List, Type, TypeVar, Generic, ClassVar, Self, Dict, Any
import requests
from pydantic import (
    BaseModel,
    ConfigDict,
    AliasGenerator,
    alias_generators,
    ValidationError,
)

T = TypeVar("T", bound=BaseModel)

class GQLResponseBase(BaseModel, Generic[T]):
    """GQL Response base class."""

    MODEL_CLS: ClassVar[Type[T]] = None

    model_config = ConfigDict(
        alias_generator=AliasGenerator(validation_alias=alias_generators.to_camel)
    )

    def to_model(self) -> T:
        """Convert to model."""
        if self.MODEL_CLS is None:
            raise NotImplementedError("to_model() isn't implemented for this class.")

        try:
            return self.MODEL_CLS.model_validate(self.model_dump())
        except ValidationError as e:
            raise ValueError(
                f"Got a validation error: {e}. Check yo GQL query hoe!!!"
            ) from e


class GQLValidationError(BaseModel):
    """GQL validation error."""

    field: Optional[str] = None
    message: Optional[str] = None


class GQLMutationResult(GQLResponseBase):
    """GQL Mutation Response base class."""

    success: Optional[bool]
    error: Optional[List[GQLValidationError]] = None

class QueryableMixin(BaseModel):
    """Queryable mixin."""
    @classmethod
    def query(cls, query: str, variables: Dict[str, Any]) -> Self:
        """Query GQL endpoint."""
        resp = requests.post(
            "http://localhost:8000/graphql",
            json={
                "query": query,
                "variables": variables,
            },
            timeout=5,
        )

        if not resp.ok:
            raise ValueError(f"GQL query failed: {resp.text}")

        data = resp.json().get("data")

        if data is None:
            raise ValueError(f"Data not in query response: {resp.text}")

        return cls(**data)

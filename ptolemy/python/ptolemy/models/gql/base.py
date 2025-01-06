"""GraphQL Response model."""

from typing import List, Type, TypeVar, Generic, ClassVar, Self, Dict, Any
import requests
from pydantic import (
    BaseModel,
    ConfigDict,
    AliasGenerator,
    alias_generators,
    ValidationError,
    Field
)

T = TypeVar("T", bound=BaseModel)

class ToModelMixin(BaseModel, Generic[T]):
    """To model mixin."""
    MODEL_CLS: ClassVar[Type[T]]

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

class GQLResponseBase(BaseModel):
    """GQL Response base class."""
    model_config = ConfigDict(
        alias_generator=AliasGenerator(validation_alias=alias_generators.to_camel),
        validate_default=False
    )


class GQLValidationError(BaseModel):
    """GQL validation error."""

    field: str = Field(default=None)
    message: str = Field(default=None)


class GQLMutationResult(GQLResponseBase):
    """GQL Mutation Response base class."""

    success: bool = Field(default=None)
    error: List[GQLValidationError] = Field(default=None)

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

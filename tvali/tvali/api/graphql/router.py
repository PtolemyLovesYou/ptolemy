"""GraphQL Router."""

import strawberry
from strawberry.fastapi import GraphQLRouter
from .query import Query

schema = strawberry.Schema(query=Query)

router = GraphQLRouter(schema, path="/graphql", tags=["graphql"])

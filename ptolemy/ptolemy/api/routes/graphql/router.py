"""Router."""

from typing import Any, List
import json
import strawberry
from strawberry.fastapi import GraphQLRouter, BaseContext
import asynch

class GraphQLContext(BaseContext):
    """Context class."""
    def __init__(self, connection: asynch.Connection) -> None:
        super().__init__()

        self.conn = connection

async def get_context():
    """Get context."""
    conn = asynch.Connection(
        host="clickhouse",
        port=9000,
        database="ptolemy",
        cursor_cls=asynch.DictCursor
    )

    try:
        await conn.connect()
        return GraphQLContext(conn)
    finally:
        if conn.connected:
            await conn.close()

@strawberry.type
class Query:
    """Query."""

    @strawberry.field
    async def query(self, info: strawberry.Info[GraphQLContext, Any], query: str) -> str:
        """Execute a query."""
        async with info.context.conn.cursor() as cursor:
            await cursor.execute(query)
            results: List[dict] = await cursor.fetchall()

        return json.dumps(results)

    @strawberry.field
    def health(self) -> str:
        """Return the health status as a string."""
        return "OK!"


schema = strawberry.Schema(query=Query)

router = GraphQLRouter(
    schema,
    path="/graphql",
    tags=["graphql"],
    context_getter=get_context,
)

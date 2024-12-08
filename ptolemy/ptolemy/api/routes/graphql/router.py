"""Router."""

import strawberry
from strawberry.fastapi import GraphQLRouter, BaseContext
import clickhouse_connect as click
from clickhouse_connect.driver.asyncclient import AsyncClient

class GraphQLContext(BaseContext):
    """Context class."""
    def __init__(self, connection: AsyncClient) -> None:
        super().__init__()

        self.conn = connection

async def get_context():
    """Get context."""
    conn = None

    try:
        conn = await click.create_async_client(
            host="clickhouse",
            port=8123,
            database="ptolemy"
        )
        return GraphQLContext(conn)
    finally:
        if conn is not None:
            conn.close()

@strawberry.type
class Query:
    """Query."""

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

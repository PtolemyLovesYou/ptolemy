"""GraphQL query."""
import strawberry

@strawberry.type
class Query:
    """GraphQL Query."""
    @strawberry.field
    def health(self) -> str:
        """Health check

        Returns:
            str: OK
        """
        return "OK"

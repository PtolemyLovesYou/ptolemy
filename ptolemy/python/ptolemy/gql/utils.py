"""GQL Utilities."""
from importlib.resources import read_text

def get_gql_query(pkg, name: str) -> str:
    """Get gql query."""
    return read_text(pkg, f"{name}.gql")

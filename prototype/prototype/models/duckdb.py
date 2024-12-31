"""DuckDB connection."""
import os
from importlib import resources
import jinja2 as j2
from pydantic import BaseModel, ConfigDict
import duckdb
from .. import sql

J2_ENV = j2.Environment()

def get_template(name: str) -> j2.Template:
    """Get SQL template."""
    template = resources.read_text(sql, name)
    return J2_ENV.from_string(template)

class DuckDB(BaseModel):
    """DuckDB connection."""
    model_config = ConfigDict(
        arbitrary_types_allowed=True,
    )

    conn: duckdb.DuckDBPyConnection

    @classmethod
    def init(cls) -> 'DuckDB':
        """Create DuckDB connection."""
        conn = duckdb.connect(":memory:")
        setup_script = (
            get_template("postgres_conn.sql.j2")
            .render(
                postgres_host=os.getenv("POSTGRES_HOST", "postgres"),
                postgres_port=os.getenv("POSTGRES_PORT", "5432"),
                postgres_user=os.getenv("POSTGRES_USER", "postgres"),
                postgres_password=os.getenv("POSTGRES_PASSWORD", "postgres"),
                postgres_db=os.getenv("POSTGRES_DB", "ptolemy"),
            )
            )

        conn.execute(setup_script)
        return cls(conn=conn)

"""TestClient."""
import pytest
from tvali.api.core.lifespan import lifespan
from tvali.api.main import app

@pytest.mark.asyncio
async def test_lifespan():
    """
    Test the lifespan function.

    This test will verify that the lifespan function will create tables
    in the database when called with the app object.

    """
    
    async with lifespan(app):
        pass

"""Config."""
from typing import Optional, ClassVar
from pydantic import BaseModel
from enum import StrEnum

class ClientType(StrEnum):
    """Client Types."""
    CONSOLE = 'console'
    API = 'api'

class TvaliConfig(BaseModel):
    """Tvali Config."""
    _config : ClassVar['TvaliConfig'] = None

    report_to: ClientType
    url: Optional[str] = None
    api_key: Optional[str] = None

def init(
    report_to: ClientType = ClientType.CONSOLE,
    url: Optional[str] = None,
    api_key: Optional[str] = None
) -> None:
    """
    Initialize the TvaliConfig object.

    Args:
        report_to (ClientType, optional): The client type to report to. Defaults to ClientType.CONSOLE.
        url (Optional[str], optional): The URL to report to. Defaults to None.
        api_key (Optional[str], optional): The API key for the client. Defaults to None.

    Returns:
        None
    """
    TvaliConfig._config = TvaliConfig(
        report_to=report_to,
        url=url,
        api_key=api_key
    )

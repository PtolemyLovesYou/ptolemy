"""Config."""
from typing import Optional, ClassVar, Callable
import functools
from pydantic import BaseModel
from .client.types import ClientType

class Config(BaseModel):
    initialized: ClassVar[bool] = False

    version: ClassVar[Optional[str]] = None
    client_type: ClassVar[str] = None

def init(
    client_type: ClientType,
    version: Optional[str] = None,
) -> None:
    Config.version = version

    Config.initialized = True
    Config.client_type = client_type

def require_initialize(func: Callable) -> Callable:
    """
    A decorator that ensures the decorated function can only be called after
    tvali.init() has been called.

    If the decorated function is called before tvali.init(), a RuntimeError
    is raised.
    """
    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        """
        A wrapper that ensures the wrapped function can only be called after
        tvali.init() has been called.

        If the wrapped function is called before tvali.init(), a RuntimeError
        is raised.
        """
        if not Config.initialized:
            raise RuntimeError("You must call tvali.init() before using this functionality.")

        return func(*args, **kwargs)

    return wrapper

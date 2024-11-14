"""Function for requiring extras."""
from typing import List
import functools
import pkg_resources

def require(*packages: str, required_extras: List[str]):
    """
    Function for requiring extras.

    Usage:
    >>> @require("requests", required_extras=["http"])
    ... def fetch_url(url):
    ...     import requests
    ...     return requests.get(url)

    :param packages: List of packages that need to be installed
    :param required_extras: List of extras that need to be installed
    """
    def decorator(func):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            try:
                pkg_resources.require(*packages)
                return func(*args, **kwargs)
            except pkg_resources.DistributionNotFound as e:
                raise ImportError(
                    f"This functionality requires the following extras to be installed: {required_extras}. To install, run `pip install tvali[{','.join(required_extras)}]`"
                    ) from e

        return wrapper

    return decorator

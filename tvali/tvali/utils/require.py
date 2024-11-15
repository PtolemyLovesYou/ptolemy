"""Function for requiring extras."""
from typing import List, Callable, Any
import functools
import pkg_resources

def require(
    *packages: str,
    required_extras: List[str]
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """
    Decorator function for requiring extras.

    Args:
        packages (List[str]): List of packages to check for installation.
        required_extras (List[str]): List of extras to require for installation.

    Returns:
        Callable[[Callable[..., Any]], Callable[..., Any]]: Decorator function to be used.
    """
    def decorator(func) -> Callable[..., Any]:
        @functools.wraps(func)
        def wrapper(*args, **kwargs) -> Any:
            try:
                # Check if the required packages are installed
                pkg_resources.require(*packages)
                return func(*args, **kwargs)
            except pkg_resources.DistributionNotFound as e:
                # Raise an error if the required extras are not installed
                raise ImportError(
                    f"This functionality requires the following extras to be installed: {required_extras}. To install, run `pip install tvali[{','.join(required_extras)}]` or `poetry install -E {' -E '.join(required_extras)}"
                    ) from e

        return wrapper

    return decorator

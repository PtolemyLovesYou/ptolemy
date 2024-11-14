"""Config"""

import os


def get_env(
    key: str, default: str = None, return_type: type = str, optional: bool = False
) -> str:
    """Get environment variable"""
    value = os.getenv(key)

    if value is None:
        if not optional:
            raise ValueError(f"Environment variable {key} is required")
        return default

    return return_type(value)


POSTGRES_USER = get_env("POSTGRES_USER")
POSTGRES_PASSWORD = get_env("POSTGRES_PASSWORD")
POSTGRES_DB = get_env("POSTGRES_DB")
POSTGRES_HOST = get_env("POSTGRES_HOST")
POSTGRES_PORT = get_env("POSTGRES_PORT", return_type=int)

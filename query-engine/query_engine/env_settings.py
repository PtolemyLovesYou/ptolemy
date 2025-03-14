"""Env settings."""

import os

REDIS_HOST = os.getenv("REDIS_HOST", "localhost")
REDIS_PORT = int(os.getenv("REDIS_PORT", "6379"))
REDIS_DB = int(os.getenv("REDIS_DB", "0"))

STREAM_NAME = os.getenv("STREAM_NAME", "ptolemy:query")
GROUP_NAME = os.getenv("GROUP_NAME", "ptolemy:query")

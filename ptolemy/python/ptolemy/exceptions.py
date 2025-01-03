"""Exceptions."""


class EngineError(Exception):
    """Base exception for engine-related errors."""


class PtolemyConnectionError(EngineError):
    """Raised when connection to the GRPC client fails."""


class PublishError(EngineError):
    """Raised when publishing records fails."""

"""Types."""
from typing import TypeVar, Union, Dict, Any, Optional, Type
from types import TracebackType
from typing_extensions import Annotated
from uuid import UUID, uuid4
import time
import traceback
from enum import StrEnum
from pydantic import (
    BeforeValidator,
    PlainSerializer,
    ValidationInfo,
    BaseModel,
    ConfigDict,
    Field,
    ValidationError
    )

class LogType(StrEnum):
    """Log Types."""
    INPUT = 'input'
    OUTPUT = 'output'
    FEEDBACK = 'feedback'
    METADATA = 'metadata'

T = TypeVar("T")

def safe_json(value: Any) -> bool:
    if value is None:
        return True
    if isinstance(value, (bool, int, float)):
        return True
    if isinstance(value, str):
        return True
    if isinstance(value, (tuple, list)):
        return all(safe_json(v) for v in value)
    if isinstance(value, dict):
        return all(isinstance(k, str) and safe_json(v) for k, v in value.items())

    return False

def validate_io(value: dict) -> dict:
    if not safe_json(value):
        raise ValueError(f'Invalid IO: {value}')

    return value

def validate_id(value: Union[UUID, str, None], info: ValidationInfo) -> UUID:
    if isinstance(value, UUID) or value is None:
        return value

    if isinstance(value, str):
        return UUID(value)

    raise ValueError(f"Unable to coerce to UUID: {value}")

ID = Annotated[
    UUID,
    BeforeValidator(validate_id),
    PlainSerializer(str, return_type=str, when_used='always')
    ]

IO = Annotated[
    dict[str, Any],
    BeforeValidator(validate_io)
    ]

Parameters = IO
Input = IO
Output = IO
Feedback = IO
Metadata = Dict[str, str]

class Record(BaseModel):
    """Records base class."""
    model_config = ConfigDict(validate_assignment=True)

    idx: ID = Field(default_factory=uuid4, alias='id')
    name: str
    parameters: Parameters = Field(default_factory=dict)
    start_time: Optional[float] = None
    end_time: Optional[float] = None
    error_type: Optional[str] = None
    error_value: Optional[str] = None
    inputs: Input = Field(default_factory=dict)
    outputs: Output = Field(default_factory=dict)
    feedback: Feedback = Field(default_factory=dict)
    metadata: Metadata = Field(default_factory=dict)

    def start(self) -> None:
        """Start record."""
        if self.start_time is not None:
            raise ValueError('Record already started')

        self.start_time = time.time()

    def end(self) -> None:
        """End record."""
        if self.end_time is not None:
            raise ValueError('Record already ended')

        self.end_time = time.time()

    def __enter__(self) -> 'Record':
        self.start()
        return self

    def __exit__(
        self,
        exc_type: Optional[Type[BaseException]],
        exc_val: Optional[BaseException],
        exc_tb: Optional[TracebackType],
    ) -> None:
        """End record.

        Args:
            exc_type: Exception type.
            exc_val: Exception value.
            exc_tb: Exception traceback.
        """
        self.end()
        if exc_type is not None:
            self.error_type = exc_type.__name__

        if exc_tb is not None:
            self.error_value = ''.join(traceback.format_tb(exc_tb)).strip()

    @classmethod
    def new(cls, name: str, parameters: Optional[Parameters] = None, **kwargs: Any) -> 'Record':
        """Create a new record.
        
        Args:
            name (str): The name of the record.
            parameters (Optional[Parameters], optional): The parameters of the record. Defaults to None.
            **kwargs: Additional kwargs to pass to Record constructor.
            
        Returns:
            Record: A new record.
        """
        params = {'name': name}
        
        if parameters:
            params['parameters'] = parameters

        return cls(**(params | kwargs))

    def log_io(
        self,
        inputs: Optional[Input] = None,
        outputs: Optional[Output] = None,
        feedback: Optional[Output] = None,
        metadata: Optional[Metadata] = None
    ) -> None:
        """Log inputs, outputs, feedback, and metadata.

        Args:
            inputs: Inputs to log.
            outputs: Outputs to log.
            feedback: Feedback to log.
            metadata: Metadata to log.

        Raises:
            ValueError: If inputs, outputs, feedback, or metadata are invalid.
        """
        self._log_dict(inputs, LogType.INPUT)
        self._log_dict(outputs, LogType.OUTPUT)
        self._log_dict(feedback, LogType.FEEDBACK)
        self._log_dict(metadata, LogType.METADATA)

    def _log_dict(self, new: dict, name: LogType) -> None:
        """Log a dictionary.

        Args:
            new: New dictionary to log.
            name: Name of the dictionary.

        Raises:
            ValueError: If new is invalid.
        """
        if new is None:
            return

        try:
            if name == LogType.INPUT:
                self.inputs |= new
            elif name == LogType.OUTPUT:
                self.outputs |= new
            elif name == LogType.FEEDBACK:
                self.feedback |= new
            elif name == LogType.METADATA:
                self.metadata |= new
        except ValidationError as e:
            if name == LogType.METADATA:
                raise ValueError(f'Invalid {name}. Keys and values must be strings.') from e
            else:
                raise ValueError(f'Invalid {name}. Keys must be strings.') from e

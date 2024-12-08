"""Query utilities for GraphQL."""
from typing import List, Self, Callable, Any, ClassVar
import abc
import functools
from importlib import resources
import jinja2 as j2
from pydantic import BaseModel, PrivateAttr, RootModel
from . import sql
from ....utils import Tier, LogType

DEFAULT_LIMIT = 100

@functools.lru_cache(maxsize=16)
def _template(template_id: str) -> str:
    with resources.open_text(sql, f"{template_id}.sql.j2") as f:
        return f.read()

def set_once(func: Callable[['QueryBuilder', Any], Any]) -> Callable[['QueryBuilder', Any], Any]:
    """Set once decorator."""
    @functools.wraps(func)
    def wrapper(self: 'QueryBuilder', *args, **kwargs):
        if getattr(self, f"_{func.__name__}") is not None:
            raise ValueError(f"{func.__name__} has already been set")

        return func(self, *args, **kwargs)

    return wrapper

QueryFilter = RootModel[str]

class Operator:
    """Operator."""
    @staticmethod
    def eq(col: str, value: Any) -> QueryFilter:
        """Equals."""
        return QueryFilter(f"{col} = {value}")

    @staticmethod
    def ne(col: str, value: Any) -> QueryFilter:
        """Doesn't equal."""
        return QueryFilter(f"{col} != {value}")

    @staticmethod
    def gt(col: str, value: Any) -> QueryFilter:
        """Greater than."""
        return QueryFilter(f"{col} > {value}")

    @staticmethod
    def lt(col: str, value: Any) -> QueryFilter:
        """Less than."""
        return QueryFilter(f"{col} < {value}")

    @staticmethod
    def is_null(col: str) -> QueryFilter:
        """Is null."""
        return QueryFilter(f"isNull({col})")

    @staticmethod
    def not_null(col: str) -> QueryFilter:
        """Is not null."""
        return QueryFilter(f"isNotNull({col})")

    @staticmethod
    def like(col: str, value: str) -> QueryFilter:
        """Like operator."""
        return QueryFilter(f"like({col}, {value})")

    @staticmethod
    def ilike(col: str, value: str) -> QueryFilter:
        """Ilike operator."""
        return QueryFilter(f"ilike({col}, {value})")

    @staticmethod
    def in_(col: str, values: List[Any]) -> QueryFilter:
        """IN operator."""
        return QueryFilter(f"{col} IN {values}")

    @staticmethod
    def notin(col: str, values: List[Any]) -> QueryFilter:
        """NOT IN operator."""
        return QueryFilter(f"{col} NOT IN {values}")

    @staticmethod
    def match(col: str, regexp: str) -> QueryFilter:
        """Regexp match operator."""
        return QueryFilter(f"match({col}, {regexp})")

    @staticmethod
    def and_(*args: QueryFilter) -> QueryFilter:
        """AND operator."""
        return QueryFilter(' AND '.join(i.model_dump() for i in args))

    @staticmethod
    def or_(*args: QueryFilter) -> QueryFilter:
        """OR operator."""
        return QueryFilter(' OR '.join(i.model_dump() for i in args))

class QueryBuilder(BaseModel, abc.ABC):
    """Query builder."""
    TEMPLATE_ID: ClassVar[str]
    J2_ENV: ClassVar[j2.Environment] = j2.Environment()

    _limit: int = PrivateAttr(default=100)
    _offset: int = PrivateAttr(default=0)
    _columns: List[str] = PrivateAttr(default=None)
    _filter: QueryFilter = PrivateAttr(default=None)

    @classmethod
    def template(cls) -> j2.Template:
        """Get template."""
        return cls.J2_ENV.from_string(
            _template(cls.TEMPLATE_ID)
            )

    @set_once
    def limit(self, limit: int) -> Self:
        """Set limit."""
        if limit < 1 or limit > 1000:
            raise ValueError("limit must be between 1 and 1000")

        self._limit = limit

        return self

    @set_once
    def offset(self, offset: int) -> Self:
        """Set offset."""
        self._offset = offset

        return self

    def bump(self, offset: int) -> Self:
        """Bump offset."""
        self._offset += offset

        return self

    @set_once
    def columns(self, *columns: str) -> Self:
        """Set columns."""
        self._columns = columns

        return self

    @set_once
    def filter(self, filters: QueryFilter) -> Self:
        """Set filters."""
        self._filter = filters

        return self

    def base_query_args(self) -> dict[str, Any]:
        """Get query args."""
        args = {
            'limit': self._limit,
            'offset': self._offset,
            'columns': self._columns,
        }

        if self._filter:
            args['filters'] = self._filter.model_dump()

        return args

    @abc.abstractmethod
    def query_args(self) -> dict[str, Any]:
        """Get query args."""

    def build(self) -> str:
        """Build."""
        if not self._columns:
            raise ValueError("columns must be set")

        return self.template().render(
            **self.query_args(),
            **self.base_query_args()
            )

class RecordQuery(QueryBuilder):
    """Record query."""
    TEMPLATE_ID = "record"

    _tier: Tier = PrivateAttr(default=None)
    _log_type: LogType = PrivateAttr(default=None)

    @set_once
    def tier(self, tier: Tier) -> Self:
        """Set tier."""
        self._tier = tier

        return self

    @set_once
    def log_type(self, log_type: LogType) -> Self:
        """Set log type."""
        self._log_type = log_type

        return self

    def query_args(self) -> dict[str, Any]:
        return {
            'tier': self._tier,
            'log_type': self._log_type,
        }

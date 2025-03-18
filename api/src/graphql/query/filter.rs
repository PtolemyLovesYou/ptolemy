use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Default, InputObject)]
pub struct EventFilter {
    pub id: Option<UuidFilter>,
    pub name: Option<StringFilter>,
    pub version: Option<StringFilter>,
    pub environment: Option<StringFilter>,
}

#[derive(Debug, Default, InputObject)]
pub struct RuntimeFilter {
    pub start_time: Option<DateTimeFilter>,
    pub end_time: Option<DateTimeFilter>,
    pub error_type: Option<StringFilter>,
    pub error_content: Option<StringFilter>,
}

#[derive(Debug, Default, InputObject)]
pub struct StringFilter {
    pub eq: Option<String>,
    pub ne: Option<String>,
    #[graphql(name = "in")]
    pub in_: Option<Vec<String>>,
    pub nin: Option<Vec<String>>,
    pub like: Option<String>,
    pub ilike: Option<String>,
    pub is_null: Option<bool>,
}

#[derive(Debug, Default, InputObject)]
pub struct UuidFilter {
    pub eq: Option<Uuid>,
    pub ne: Option<Uuid>,
    #[graphql(name = "in")]
    pub in_: Option<Vec<Uuid>>,
    pub nin: Option<Vec<Uuid>>,
    pub is_null: Option<bool>,
}

#[derive(Debug, Default, InputObject)]
pub struct BoolFilter {
    pub eq: Option<bool>,
    pub ne: Option<bool>,
    pub is_null: Option<bool>,
}

#[derive(Debug, Default, InputObject)]
pub struct DateTimeFilter {
    pub lt: Option<DateTime<Utc>>,
    pub gt: Option<DateTime<Utc>>,
    pub is_null: Option<bool>,
}

#[macro_export]
macro_rules! search_filter {
    ($query:expr, $filter:expr, $event_tier:ident, Event) => {{
        let query = $query; // Use shadowing instead of reassignment

        let query = if let Some(id) = &$filter.id {
            $crate::search_filter!(
                query,
                crate::generated::records_schema::$event_tier::id,
                id,
                Uuid
            )
        } else {
            query
        };

        let query = if let Some(name) = &$filter.name {
            $crate::search_filter!(
                query,
                crate::generated::records_schema::$event_tier::name,
                name,
                String
            )
        } else {
            query
        };

        let query = if let Some(version) = &$filter.version {
            $crate::search_filter!(
                query,
                crate::generated::records_schema::$event_tier::version,
                version,
                String
            )
        } else {
            query
        };

        let query = if let Some(environment) = &$filter.environment {
            $crate::search_filter!(
                query,
                crate::generated::records_schema::$event_tier::environment,
                environment,
                String
            )
        } else {
            query
        };

        query
    }};
    ($query:expr, $filter:expr, Runtime) => {{
        let query = $query; // Use shadowing instead of reassignment

        let query = if let Some(start_time) = &$filter.start_time {
            $crate::search_filter!(
                query,
                crate::generated::records_schema::runtime::start_time,
                start_time,
                DateTime
            )
        } else {
            query
        };

        let query = if let Some(end_time) = &$filter.end_time {
            $crate::search_filter!(
                query,
                crate::generated::records_schema::runtime::end_time,
                end_time,
                DateTime
            )
        } else {
            query
        };

        let query = if let Some(error_type) = &$filter.error_type {
            $crate::search_filter!(
                query,
                crate::generated::records_schema::runtime::error_type,
                error_type,
                String
            )
        } else {
            query
        };

        let query = if let Some(error_content) = &$filter.error_content {
            $crate::search_filter!(
                query,
                crate::generated::records_schema::runtime::error_content,
                error_content,
                String
            )
        } else {
            query
        };

        query
    }};
    ($query:expr, $column:expr, $filter:expr, DateTime) => {{
        // let query = $crate::search_filter!($query, $column, $filter);
        let query = $query;

        let query = if let Some(lt) = &$filter.lt {
            query.filter($column.lt(lt))
        } else {
            query
        };

        let query = if let Some(gt) = &$filter.gt {
            query.filter($column.gt(gt))
        } else {
            query
        };

        query
    }};
    ($query:expr, $column:expr, $filter:expr, Bool) => {{
        $crate::search_filter!($query, $column, $filter)
    }};
    ($query:expr, $column:expr, $filter:expr, Uuid) => {{
        let query = $crate::search_filter!($query, $column, $filter);

        let query = if let Some(in_) = &$filter.in_ {
            query.filter($column.eq_any(in_))
        } else {
            query
        };

        let query = if let Some(nin) = &$filter.nin {
            query.filter(diesel::dsl::not($column.eq_any(nin)))
        } else {
            query
        };

        query
    }};
    ($query:expr, $column:expr, $filter:expr, String) => {{
        let query = $crate::search_filter!($query, $column, $filter);

        let query = if let Some(in_) = &$filter.in_ {
            query.filter($column.eq_any(in_))
        } else {
            query
        };

        let query = if let Some(nin) = &$filter.nin {
            query.filter(diesel::dsl::not($column.eq_any(nin)))
        } else {
            query
        };

        let query = if let Some(like) = &$filter.like {
            query.filter($column.like(like))
        } else {
            query
        };

        let query = if let Some(ilike) = &$filter.ilike {
            query.filter($column.ilike(ilike))
        } else {
            query
        };

        query
    }};
    ($query:expr, $column:expr, $filter:expr) => {{
        let query = $query; // Use shadowing instead of reassignment

        let query = if let Some(eq) = &$filter.eq {
            query.filter($column.eq(eq))
        } else {
            query
        };

        let query = if let Some(ne) = &$filter.ne {
            query.filter($column.ne(ne))
        } else {
            query
        };

        let query = if let Some(is_null) = &$filter.is_null {
            if *is_null {
                query.filter($column.is_null())
            } else {
                query.filter($column.is_not_null())
            }
        } else {
            query
        };

        query
    }};
}

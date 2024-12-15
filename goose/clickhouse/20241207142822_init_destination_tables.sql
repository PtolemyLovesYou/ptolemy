-- +goose envsub on

-- +goose Up

-- ## Create destination tables ########################################

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.system_event
    (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    name String,
    parameters JSON,
    version String,
    environment String,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree ORDER BY (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_event (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    name String,
    parameters JSON,
    version String,
    environment String,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);



create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.component_event (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    name String,
    parameters JSON,
    version String,
    environment String,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);



create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_event (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    name String,
    parameters JSON,
    version String,
    environment String,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);



create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.system_runtime (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    start_time DateTime64(6) NOT NULL,
    end_time DateTime64(6) NOT NULL,
    error_type String,
    error_content String,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_runtime (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    start_time DateTime64(6) NOT NULL,
    end_time DateTime64(6) NOT NULL,
    error_type String,
    error_content String,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.component_runtime (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    start_time DateTime64(6) NOT NULL,
    end_time DateTime64(6) NOT NULL,
    error_type String,
    error_content String,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_runtime (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    start_time DateTime64(6) NOT NULL,
    end_time DateTime64(6) NOT NULL,
    error_type String,
    error_content String,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.system_input (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_input (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.component_input (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_input (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.system_output (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_output (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.component_output (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_output (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.system_feedback (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_feedback (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.component_feedback (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_feedback (
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.system_metadata(
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value String NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_metadata(
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value String NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.component_metadata(
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value String NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_metadata(
    parent_id UUID NOT NULL,
    id UUID NOT NULL,
    field_name String NOT NULL,
    field_value String NOT NULL,
    created_at DateTime64(6) NOT NULL
) engine = MergeTree order by (parent_id, id, created_at);

-- #####################################################################

-- +goose Down
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.system_event;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_event;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.component_event;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_event;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.system_runtime;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_runtime;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.component_runtime;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_runtime;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.system_input;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_input;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.component_input;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_input;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.system_output;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_output;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.component_output;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_output;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.system_feedback;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_feedback;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.component_feedback;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_feedback;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.system_metadata;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_metadata;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.component_metadata;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_metadata;

-- +goose envsub off

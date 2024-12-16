-- +goose envsub on
-- +goose Up

-- ## Create normalized records table ##################################

create database if not exists ${PTOLEMY_CLICKHOUSE_DATABASE};

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__records (
    -- Universal fields
    tier Enum('SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4) NOT NULL,
    log_type Enum('EVENT' = 1, 'RUNTIME' = 2, 'INPUT' = 3, 'OUTPUT' = 4, 'FEEDBACK' = 5, 'METADATA' = 6) NOT NULL,
    parent_id String NOT NULL,
    id String NOT NULL,

    -- Event fields
    name String,
    parameters JSON,
    version String,
    environment String,

    -- Runtime fields
    start_time String,
    end_time String,
    error_type String,
    error_value String,

    -- IO field_name
    field_name String,

    -- Variant field values (for Input, Output, Feedback)
    field_value_var Variant(String, Int64, Float64, Bool, UUID, JSON),

    -- Metadata fields
    field_value_str String,

    -- Created at field
    created_at DateTime64(9) default now64()

) ENGINE = MergeTree() ORDER BY (log_type, tier, created_at);

-- #####################################################################

-- +goose Down

drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__records;
drop database if exists ${PTOLEMY_CLICKHOUSE_DATABASE};

-- +goose envsub off

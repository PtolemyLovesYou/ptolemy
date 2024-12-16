-- +goose envsub on
-- +goose Up

-- ## Create normalized records table ##################################

create database if not exists ${PTOLEMY_CLICKHOUSE_DATABASE};

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__event (
    tier Enum('SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4),
    parent_id UUID,
    id UUID,
    name String,
    parameters JSON,
    version String,
    environment String,
    created_at DateTime64(6) default now64()

) ENGINE = MergeTree() ORDER BY (tier, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__runtime (
    tier Enum('SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4),
    parent_id UUID,
    id UUID,
    start_time String,
    end_time String,
    error_type String,
    error_content String,
    created_at DateTime64(6) default now64()

) ENGINE = MergeTree() ORDER BY (tier, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__input (
    tier Enum('SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4),
    parent_id UUID,
    id UUID,
    field_name String,
    field_value Variant(String, Int64, Float64, Bool, JSON),
    created_at DateTime64(6) default now64()

) ENGINE = MergeTree() ORDER BY (tier, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__output (
    tier Enum('SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4),
    parent_id UUID,
    id UUID,
    name String,
    parameters JSON,
    version String,
    environment String,
    field_name String,
    field_value Variant(String, Int64, Float64, Bool, JSON),
    created_at DateTime64(6) default now64()

) ENGINE = MergeTree() ORDER BY (tier, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__feedback (
    tier Enum('SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4),
    parent_id UUID,
    id UUID,
    field_name String,
    field_value Variant(String, Int64, Float64, Bool, JSON),
    created_at DateTime64(6) default now64()

) ENGINE = MergeTree() ORDER BY (tier, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__metadata (
    tier Enum('SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4),
    parent_id UUID,
    id UUID,
    field_name String,
    field_value Variant(String, Int64, Float64, Bool, JSON),
    created_at DateTime64(6) default now64()

) ENGINE = MergeTree() ORDER BY (tier, created_at);

-- #####################################################################

-- +goose Down

drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__records;
drop database if exists ${PTOLEMY_CLICKHOUSE_DATABASE};

-- +goose envsub off

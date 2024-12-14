-- +goose envsub on
-- +goose Up

-- ## Create normalized records table ##################################

create database if not exists ${PTOLEMY_CLICKHOUSE_DATABASE};

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__event (
    tier Enum('UNDECLARED_TIER' = 0, 'SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4) NOT NULL,
    parent_id String NOT NULL,
    id String NOT NULL,
    name String,
    parameters JSON,
    version String,
    environment String,
    created_at DateTime64(9) default now64()
) ENGINE = MergeTree() ORDER BY (tier, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__runtime (
    tier Enum('UNDECLARED_TIER' = 0, 'SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4) NOT NULL,
    parent_id String NOT NULL,
    id String NOT NULL,
    start_time String,
    end_time String,
    error_type String,
    error_value String,
    created_at DateTime64(9) default now64()
) ENGINE = MergeTree() ORDER BY (tier, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__input (
    tier Enum('UNDECLARED_TIER' = 0, 'SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4) NOT NULL,
    parent_id String NOT NULL,
    id String NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    is_json Bool NOT NULL,
    created_at DateTime64(9) default now64()
) ENGINE = MergeTree() ORDER BY (tier, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__output (
    tier Enum('UNDECLARED_TIER' = 0, 'SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4) NOT NULL,
    parent_id String NOT NULL,
    id String NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    is_json Bool NOT NULL,
    created_at DateTime64(9) default now64()
) ENGINE = MergeTree() ORDER BY (tier, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__feedback (
    tier Enum('UNDECLARED_TIER' = 0, 'SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4) NOT NULL,
    parent_id String NOT NULL,
    id String NOT NULL,
    field_name String NOT NULL,
    field_value Variant(String, Decimal64(18), Bool, UUID, JSON) NOT NULL,
    is_json Bool NOT NULL,
    created_at DateTime64(9) default now64()
) ENGINE = MergeTree() ORDER BY (tier, created_at);

create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__metadata (
    tier Enum('UNDECLARED_TIER' = 0, 'SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4) NOT NULL,
    parent_id String NOT NULL,
    id String NOT NULL,
    field_name String NOT NULL,
    field_value String NOT NULL,
    created_at DateTime64(9) default now64()
) ENGINE = MergeTree() ORDER BY (tier, created_at);

-- #####################################################################

-- +goose Down

drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__event;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__runtime;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__input;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__output;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__feedback;
drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.stg__metadata;

drop database if exists ${PTOLEMY_CLICKHOUSE_DATABASE};

-- +goose envsub off

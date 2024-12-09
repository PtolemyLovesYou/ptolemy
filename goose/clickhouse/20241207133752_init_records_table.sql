-- +goose envsub on
-- +goose Up

-- ## Create normalized records table ##################################

create database if not exists ${PTOLEMY_CLICKHOUSE_DATABASE};



create or replace table ${PTOLEMY_CLICKHOUSE_DATABASE}.records (
    tier Enum('UNDECLARED_TIER' = 0, 'SYSTEM' = 1, 'SUBSYSTEM' = 2, 'COMPONENT' = 3, 'SUBCOMPONENT' = 4) NOT NULL,
    log_type Enum('UNDECLARED_LOG_TYPE' = 0, 'EVENT' = 1, 'RUNTIME' = 2, 'INPUT' = 3, 'OUTPUT' = 4, 'FEEDBACK' = 5, 'METADATA' = 6) NOT NULL,
    parent_id String NOT NULL,
    id String NOT NULL,
    name String,
    parameters String,
    version String,
    environment String,
    start_time String,
    end_time String,
    error_type String,
    error_content String,
    field_name String,
    field_value String,
    created_at DateTime64(9) default now64()
) ENGINE = MergeTree() ORDER BY (tier, log_type, created_at);

-- #####################################################################

-- +goose Down

drop table if exists ${PTOLEMY_CLICKHOUSE_DATABASE}.records;
drop database if exists ${PTOLEMY_CLICKHOUSE_DATABASE};

-- +goose envsub off

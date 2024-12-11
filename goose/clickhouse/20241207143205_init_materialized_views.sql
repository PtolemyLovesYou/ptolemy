-- +goose envsub on

-- +goose Up

-- ## Create materialized views ########################################

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.system_event_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.system_event as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    name,
    CAST(COALESCE(parameters, '{}') as JSON) as parameters,
    version,
    environment,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 1 and log_type = 1
SETTINGS
    enable_json_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_event_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_event as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    name,
    CAST(COALESCE(parameters, '{}') as JSON) as parameters,
    version,
    environment,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 2 and log_type = 1
SETTINGS
    enable_json_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.component_event_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.component_event as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    name,
    CAST(COALESCE(parameters, '{}') as JSON) as parameters,
    version,
    environment,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 3 and log_type = 1
SETTINGS
    enable_json_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_event_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_event as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    name,
    CAST(COALESCE(parameters, '{}') as JSON) as parameters,
    version,
    environment,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 4 and log_type = 1
SETTINGS
    enable_json_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.system_runtime_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.system_runtime as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    toDateTime64(start_time, 9) as start_time,
    toDateTime64(end_time, 9) as end_time,
    error_type,
    error_content,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 1 and log_type = 2;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_runtime_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_runtime as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    toDateTime64(start_time, 9) as start_time,
    toDateTime64(end_time, 9) as end_time,
    error_type,
    error_content,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 2 and log_type = 2;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.component_runtime_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.component_runtime as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    toDateTime64(start_time, 9) as start_time,
    toDateTime64(end_time, 9) as end_time,
    error_type,
    error_content,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 3 and log_type = 2;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_runtime_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_runtime as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    toDateTime64(start_time, 9) as start_time,
    toDateTime64(end_time, 9) as end_time,
    error_type,
    error_content,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 4 and log_type = 2;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.system_input_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.system_input as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value::Dynamic as field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 1 and log_type = 3
SETTINGS
    enable_dynamic_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_input_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_input as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value::Dynamic as field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 2 and log_type = 3
SETTINGS
    enable_dynamic_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.component_input_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.component_input as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value::Dynamic as field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 3 and log_type = 3
SETTINGS
    enable_dynamic_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_input_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_input as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value::Dynamic as field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 4 and log_type = 3
SETTINGS
    enable_dynamic_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.system_output_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.system_output as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value::Dynamic as field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 1 and log_type = 4
SETTINGS
    enable_dynamic_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_output_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_output as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value::Dynamic as field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 2 and log_type = 4
SETTINGS
    enable_dynamic_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.component_output_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.component_output as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value::Dynamic as field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 3 and log_type = 4
SETTINGS
    enable_dynamic_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_output_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_output as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value::Dynamic as field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 4 and log_type = 4
SETTINGS
    enable_dynamic_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.system_feedback_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.system_feedback as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value::Dynamic as field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 1 and log_type = 5
SETTINGS
    enable_dynamic_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_feedback_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_feedback as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value::Dynamic as field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 2 and log_type = 5
SETTINGS
    enable_dynamic_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.component_feedback_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.component_feedback as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value::Dynamic as field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 3 and log_type = 5
SETTINGS
    enable_dynamic_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_feedback_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_feedback as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value::Dynamic as field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 4 and log_type = 5
SETTINGS
    enable_dynamic_type = 1;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.system_metadata_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.system_metadata as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 1 and log_type = 6;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_metadata_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_metadata as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 2 and log_type = 6;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.component_metadata_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.component_metadata as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 3 and log_type = 6;

create materialized view ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_metadata_mv to ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_metadata as
select
    toUUID(parent_id) as parent_id,
    toUUID(id) as id,
    field_name,
    field_value,
    created_at
from ${PTOLEMY_CLICKHOUSE_DATABASE}.records
where tier = 4 and log_type = 6;

-- #####################################################################

-- +goose Down

drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.system_event_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_event_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.component_input_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_input_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.system_output_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_output_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.component_output_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_output_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.system_feedback_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_feedback_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.component_feedback_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_feedback_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.system_metadata_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.subsystem_metadata_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.component_metadata_mv;
drop view ${PTOLEMY_CLICKHOUSE_DATABASE}.subcomponent_metadata_mv;

-- +goose envsub off

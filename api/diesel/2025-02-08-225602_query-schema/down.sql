-- This file should undo anything in `up.sql`
-- Drop all views in duckdb schema
drop view if exists duckdb.subcomponent_metadata;
drop view if exists duckdb.component_metadata;
drop view if exists duckdb.subsystem_metadata;
drop view if exists duckdb.system_metadata;

drop view if exists duckdb.subcomponent_feedback;
drop view if exists duckdb.component_feedback;
drop view if exists duckdb.subsystem_feedback;
drop view if exists duckdb.system_feedback;

drop view if exists duckdb.subcomponent_output;
drop view if exists duckdb.component_output;
drop view if exists duckdb.subsystem_output;
drop view if exists duckdb.system_output;

drop view if exists duckdb.subcomponent_input;
drop view if exists duckdb.component_input;
drop view if exists duckdb.subsystem_input;
drop view if exists duckdb.system_input;

drop view if exists duckdb.subcomponent_runtime;
drop view if exists duckdb.component_runtime;
drop view if exists duckdb.subsystem_runtime;
drop view if exists duckdb.system_runtime;

drop view if exists duckdb.subcomponent_event;
drop view if exists duckdb.component_event;
drop view if exists duckdb.subsystem_event;
drop view if exists duckdb.system_event;

drop view if exists duckdb.workspace_user;
drop view if exists duckdb.users;
drop view if exists duckdb.workspace;

-- Drop duckdb schema
drop schema if exists duckdb;

-- Drop all soft delete rules
drop rule if exists soft_delete_metadata on metadata;
drop rule if exists soft_delete_io on io;
drop rule if exists soft_delete_runtime on runtime;
drop rule if exists soft_delete_subcomponent_event on subcomponent_event;
drop rule if exists soft_delete_component_event on component_event;
drop rule if exists soft_delete_subsystem_event on subsystem_event;
drop rule if exists soft_delete_system_event on system_event;

-- Remove deleted_at and deletion_reason columns
alter table metadata
    drop column if exists deleted_at,
    drop column if exists deletion_reason;

alter table io
    drop column if exists deleted_at,
    drop column if exists deletion_reason;

alter table runtime
    drop column if exists deleted_at,
    drop column if exists deletion_reason;

alter table subcomponent_event
    drop column if exists deleted_at,
    drop column if exists deletion_reason;

alter table component_event
    drop column if exists deleted_at,
    drop column if exists deletion_reason;

alter table subsystem_event
    drop column if exists deleted_at,
    drop column if exists deletion_reason;

alter table system_event
    drop column if exists deleted_at,
    drop column if exists deletion_reason;

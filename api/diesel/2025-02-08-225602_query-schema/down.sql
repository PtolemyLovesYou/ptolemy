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

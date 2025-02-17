-- This file should undo anything in `up.sql`
drop rule soft_delete_metadata on metadata;
drop rule soft_delete_io on io;
drop rule soft_delete_runtime on runtime;
drop rule soft_delete_subcomponent_event on subcomponent_event;
drop rule soft_delete_component_event on component_event;
drop rule soft_delete_subsystem_event on subsystem_event;
drop rule soft_delete_system_event on system_event;
drop table runtime;
drop table io;
drop table metadata;
drop table subcomponent_event;
drop table component_event;
drop table subsystem_event;
drop table system_event;
drop type field_value_type;
drop type tier;
drop type io_type;

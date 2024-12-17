create table system_event (
id uuid primary key,
parent_id uuid not null references system_event(id),
name varchar not null,
parameters json,
version varchar(16),
environment varchar(8)
);
create table system_runtime (
id uuid primary key,
parent_id uuid not null references system_event(id),
start_time timestamp(6) not null,
end_time timestamp(6) not null,
error_type varchar,
error_value varchar
);
create table system_input_key (
id uuid primary key references system_event(id),
parent_id uuid not null,
field_name varchar not null
);
create table system_input_str (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_input_key(id),
field_value varchar not null
);
create table system_input_int (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_input_key(id),
field_value int8 not null
);
create table system_input_float (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_input_key(id),
field_value float8 not null
);
create table system_input_bool (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_input_key(id),
field_value bool not null
);
create table system_input_json (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_input_key(id),
field_value json not null
);
create table system_output_key (
id uuid primary key references system_event(id),
parent_id uuid not null,
field_name varchar not null
);
create table system_output_str (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_output_key(id),
field_value varchar not null
);
create table system_output_int (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_output_key(id),
field_value int8 not null
);
create table system_output_float (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_output_key(id),
field_value float8 not null
);
create table system_output_bool (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_output_key(id),
field_value bool not null
);
create table system_output_json (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_output_key(id),
field_value json not null
);
create table system_feedback_key (
id uuid primary key references system_event(id),
parent_id uuid not null,
field_name varchar not null
);
create table system_feedback_str (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_feedback_key(id),
field_value varchar not null
);
create table system_feedback_int (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_feedback_key(id),
field_value int8 not null
);
create table system_feedback_float (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_feedback_key(id),
field_value float8 not null
);
create table system_feedback_bool (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_feedback_key(id),
field_value bool not null
);
create table system_feedback_json (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references system_feedback_key(id),
field_value json not null
);
create table system_metadata (
id uuid primary key,
parent_id uuid not null references system_event(id),
field_name varchar not null,
field_value varchar not null
);
create table subsystem_event (
id uuid primary key,
parent_id uuid not null references subsystem_event(id),
name varchar not null,
parameters json,
version varchar(16),
environment varchar(8)
);
create table subsystem_runtime (
id uuid primary key,
parent_id uuid not null references subsystem_event(id),
start_time timestamp(6) not null,
end_time timestamp(6) not null,
error_type varchar,
error_value varchar
);
create table subsystem_input_key (
id uuid primary key references subsystem_event(id),
parent_id uuid not null,
field_name varchar not null
);
create table subsystem_input_str (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_input_key(id),
field_value varchar not null
);
create table subsystem_input_int (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_input_key(id),
field_value int8 not null
);
create table subsystem_input_float (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_input_key(id),
field_value float8 not null
);
create table subsystem_input_bool (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_input_key(id),
field_value bool not null
);
create table subsystem_input_json (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_input_key(id),
field_value json not null
);
create table subsystem_output_key (
id uuid primary key references subsystem_event(id),
parent_id uuid not null,
field_name varchar not null
);
create table subsystem_output_str (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_output_key(id),
field_value varchar not null
);
create table subsystem_output_int (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_output_key(id),
field_value int8 not null
);
create table subsystem_output_float (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_output_key(id),
field_value float8 not null
);
create table subsystem_output_bool (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_output_key(id),
field_value bool not null
);
create table subsystem_output_json (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_output_key(id),
field_value json not null
);
create table subsystem_feedback_key (
id uuid primary key references subsystem_event(id),
parent_id uuid not null,
field_name varchar not null
);
create table subsystem_feedback_str (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_feedback_key(id),
field_value varchar not null
);
create table subsystem_feedback_int (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_feedback_key(id),
field_value int8 not null
);
create table subsystem_feedback_float (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_feedback_key(id),
field_value float8 not null
);
create table subsystem_feedback_bool (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_feedback_key(id),
field_value bool not null
);
create table subsystem_feedback_json (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subsystem_feedback_key(id),
field_value json not null
);
create table subsystem_metadata (
id uuid primary key,
parent_id uuid not null references subsystem_event(id),
field_name varchar not null,
field_value varchar not null
);
create table component_event (
id uuid primary key,
parent_id uuid not null references component_event(id),
name varchar not null,
parameters json,
version varchar(16),
environment varchar(8)
);
create table component_runtime (
id uuid primary key,
parent_id uuid not null references component_event(id),
start_time timestamp(6) not null,
end_time timestamp(6) not null,
error_type varchar,
error_value varchar
);
create table component_input_key (
id uuid primary key references component_event(id),
parent_id uuid not null,
field_name varchar not null
);
create table component_input_str (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_input_key(id),
field_value varchar not null
);
create table component_input_int (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_input_key(id),
field_value int8 not null
);
create table component_input_float (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_input_key(id),
field_value float8 not null
);
create table component_input_bool (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_input_key(id),
field_value bool not null
);
create table component_input_json (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_input_key(id),
field_value json not null
);
create table component_output_key (
id uuid primary key references component_event(id),
parent_id uuid not null,
field_name varchar not null
);
create table component_output_str (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_output_key(id),
field_value varchar not null
);
create table component_output_int (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_output_key(id),
field_value int8 not null
);
create table component_output_float (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_output_key(id),
field_value float8 not null
);
create table component_output_bool (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_output_key(id),
field_value bool not null
);
create table component_output_json (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_output_key(id),
field_value json not null
);
create table component_feedback_key (
id uuid primary key references component_event(id),
parent_id uuid not null,
field_name varchar not null
);
create table component_feedback_str (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_feedback_key(id),
field_value varchar not null
);
create table component_feedback_int (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_feedback_key(id),
field_value int8 not null
);
create table component_feedback_float (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_feedback_key(id),
field_value float8 not null
);
create table component_feedback_bool (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_feedback_key(id),
field_value bool not null
);
create table component_feedback_json (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references component_feedback_key(id),
field_value json not null
);
create table component_metadata (
id uuid primary key,
parent_id uuid not null references component_event(id),
field_name varchar not null,
field_value varchar not null
);
create table subcomponent_event (
id uuid primary key,
parent_id uuid not null references subcomponent_event(id),
name varchar not null,
parameters json,
version varchar(16),
environment varchar(8)
);
create table subcomponent_runtime (
id uuid primary key,
parent_id uuid not null references subcomponent_event(id),
start_time timestamp(6) not null,
end_time timestamp(6) not null,
error_type varchar,
error_value varchar
);
create table subcomponent_input_key (
id uuid primary key references subcomponent_event(id),
parent_id uuid not null,
field_name varchar not null
);
create table subcomponent_input_str (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_input_key(id),
field_value varchar not null
);
create table subcomponent_input_int (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_input_key(id),
field_value int8 not null
);
create table subcomponent_input_float (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_input_key(id),
field_value float8 not null
);
create table subcomponent_input_bool (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_input_key(id),
field_value bool not null
);
create table subcomponent_input_json (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_input_key(id),
field_value json not null
);
create table subcomponent_output_key (
id uuid primary key references subcomponent_event(id),
parent_id uuid not null,
field_name varchar not null
);
create table subcomponent_output_str (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_output_key(id),
field_value varchar not null
);
create table subcomponent_output_int (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_output_key(id),
field_value int8 not null
);
create table subcomponent_output_float (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_output_key(id),
field_value float8 not null
);
create table subcomponent_output_bool (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_output_key(id),
field_value bool not null
);
create table subcomponent_output_json (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_output_key(id),
field_value json not null
);
create table subcomponent_feedback_key (
id uuid primary key references subcomponent_event(id),
parent_id uuid not null,
field_name varchar not null
);
create table subcomponent_feedback_str (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_feedback_key(id),
field_value varchar not null
);
create table subcomponent_feedback_int (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_feedback_key(id),
field_value int8 not null
);
create table subcomponent_feedback_float (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_feedback_key(id),
field_value float8 not null
);
create table subcomponent_feedback_bool (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_feedback_key(id),
field_value bool not null
);
create table subcomponent_feedback_json (
id uuid primary key default gen_random_uuid(),
value_id uuid not null references subcomponent_feedback_key(id),
field_value json not null
);
create table subcomponent_metadata (
id uuid primary key,
parent_id uuid not null references subcomponent_event(id),
field_name varchar not null,
field_value varchar not null
);

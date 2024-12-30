-- This file should undo anything in `up.sql`

-- For events tables
DROP INDEX idx_system_event_workspace_id;
DROP INDEX idx_subsystem_event_workspace_id;
DROP INDEX idx_component_event_workspace_id;
DROP INDEX idx_subcomponent_event_workspace_id;

-- For runtime/io/metadata tables
DROP INDEX idx_runtime_tier;
DROP INDEX idx_io_tier;
DROP INDEX idx_metadata_tier;

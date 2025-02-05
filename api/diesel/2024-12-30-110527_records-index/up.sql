-- Your SQL goes here

-- For events tables
CREATE INDEX idx_system_event_workspace_id ON system_event USING HASH (workspace_id);
CREATE INDEX idx_subsystem_event_workspace_id ON subsystem_event USING HASH (system_event_id);
CREATE INDEX idx_component_event_workspace_id ON component_event USING HASH (subsystem_event_id);
CREATE INDEX idx_subcomponent_event_workspace_id ON subcomponent_event USING HASH (component_event_id);

-- For runtime/io/metadata tables
CREATE INDEX idx_runtime_tier ON runtime USING HASH (tier);
CREATE INDEX idx_io_tier ON io USING HASH (tier);
CREATE INDEX idx_metadata_tier ON metadata USING HASH (tier);

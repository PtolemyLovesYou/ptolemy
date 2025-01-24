use crate::{
    models::records::{
        ComponentEventRecord, IORecord, MetadataRecord, RuntimeRecord, SubcomponentEventRecord,
        SubsystemEventRecord, SystemEventRecord,
    },
    generated::records_schema::{
        system_event, subsystem_event, component_event, subcomponent_event, io, metadata, runtime,
    },
    insert_obj_traits,
};
use diesel_async::RunQueryDsl;

insert_obj_traits!(SystemEventRecord, system_event);
insert_obj_traits!(SubsystemEventRecord, subsystem_event);
insert_obj_traits!(ComponentEventRecord, component_event);
insert_obj_traits!(SubcomponentEventRecord, subcomponent_event);
insert_obj_traits!(IORecord, io);
insert_obj_traits!(MetadataRecord, metadata);
insert_obj_traits!(RuntimeRecord, runtime);

use crate::{generated::records_schema::*, insert_obj_traits, models::records::*};
use diesel_async::RunQueryDsl;

insert_obj_traits!(SystemEventRecord, system_event);
insert_obj_traits!(SubsystemEventRecord, subsystem_event);
insert_obj_traits!(ComponentEventRecord, component_event);
insert_obj_traits!(SubcomponentEventRecord, subcomponent_event);
insert_obj_traits!(IORecord, io);
insert_obj_traits!(MetadataRecord, metadata);
insert_obj_traits!(RuntimeRecord, runtime);

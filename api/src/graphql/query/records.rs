use crate::{
    error::ApiError,
    generated::records_schema,
    graphql::{executor::GraphQLExecutor, state::GraphQLAppState},
    models::records::{
        ComponentEventRecord, IORecord, IoTypeEnum, MetadataRecord, RuntimeRecord,
        SubcomponentEventRecord, SubsystemEventRecord, SystemEventRecord,
    },
};
use async_graphql::{ComplexObject, Context, Result as GraphQLResult};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

macro_rules! records {
    ($obj:ident, $ctx:ident, Runtime) => {{
        let state = $ctx.data::<GraphQLAppState>()?;
        let mut conn = state.state.get_conn().await?;

        records!($obj, state, conn, Runtime)
    }};
    ($obj:ident, $ctx:ident, Input) => {{
        let state = $ctx.data::<GraphQLAppState>()?;
        let mut conn = state.state.get_conn().await?;

        records!($obj, state, conn, Input, Io)
    }};
    ($obj:ident, $ctx:ident, Output) => {{
        let state = $ctx.data::<GraphQLAppState>()?;
        let mut conn = state.state.get_conn().await?;

        records!($obj, state, conn, Output, Io)
    }};
    ($obj:ident, $ctx:ident, Feedback) => {{
        let state = $ctx.data::<GraphQLAppState>()?;
        let mut conn = state.state.get_conn().await?;

        records!($obj, state, conn, Feedback, Io)
    }};
    ($obj:ident, $ctx:ident, Metadata) => {{
        let state = $ctx.data::<GraphQLAppState>()?;
        let mut conn = state.state.get_conn().await?;

        records!($obj, state, conn, Metadata)
    }};
    ($obj:ident, $ctx:ident, $event_type:ident, $name:literal) => {{
        let state = $ctx.data::<GraphQLAppState>()?;
        let mut conn = state.state.get_conn().await?;

        records!($obj, state, conn, $event_type, $name)
    }};
    ($obj:ident, $state:ident, $conn:ident, Runtime) => {
        $crate::unchecked_executor!($state, "runtime")
            .read(async move {
                RuntimeRecord::belonging_to($obj)
                    .select(RuntimeRecord::as_select())
                    .filter(records_schema::runtime::deleted_at.is_null())
                    .get_result(&mut $conn)
                    .await
                    .map_err(|_| ApiError::GetError)
            })
            .await
            .map_err(|e| e.into())
    };
    ($obj:ident, $state:ident, $conn:ident, $io_type:ident, Io) => {
        $crate::unchecked_executor!($state, "io")
            .read_many(async move {
                IORecord::belonging_to($obj)
                    .select(IORecord::as_select())
                    .filter(
                        records_schema::io::deleted_at
                            .is_null()
                            .and(records_schema::io::io_type.eq(IoTypeEnum::$io_type)),
                    )
                    .get_results(&mut $conn)
                    .await
                    .map_err(|_| ApiError::GetError)
            })
            .await
            .map_err(|e| e.into())
    };
    ($obj:ident, $state:ident, $conn:ident, Metadata) => {
        $crate::unchecked_executor!($state, "metadata")
            .read_many(async move {
                MetadataRecord::belonging_to($obj)
                    .select(MetadataRecord::as_select())
                    .filter(records_schema::metadata::deleted_at.is_null())
                    .get_results(&mut $conn)
                    .await
                    .map_err(|_| ApiError::GetError)
            })
            .await
            .map_err(|e| e.into())
    };
    ($obj:ident, $state:ident, $conn:ident, $event_type:ident, $name:literal) => {
        $crate::unchecked_executor!($state, $name)
            .read_many(async move {
                $event_type::belonging_to($obj)
                    .select($event_type::as_select())
                    .get_results(&mut $conn)
                    .await
                    .map_err(|_| ApiError::GetError)
            })
            .await
            .map_err(|e| e.into())
    };
}

#[ComplexObject]
impl SystemEventRecord {
    async fn subsystem_events(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQLResult<Vec<SubsystemEventRecord>> {
        records!(self, ctx, SubsystemEventRecord, "subsystem_event")
    }

    async fn runtime(&self, ctx: &Context<'_>) -> GraphQLResult<RuntimeRecord> {
        records!(self, ctx, Runtime)
    }

    async fn inputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        records!(self, ctx, Input)
    }

    async fn outputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        records!(self, ctx, Output)
    }

    async fn feedback(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        records!(self, ctx, Feedback)
    }

    async fn metadata(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<MetadataRecord>> {
        records!(self, ctx, Metadata)
    }
}

#[ComplexObject]
impl SubsystemEventRecord {
    async fn component_events(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQLResult<Vec<ComponentEventRecord>> {
        records!(self, ctx, ComponentEventRecord, "component_event")
    }

    async fn runtime(&self, ctx: &Context<'_>) -> GraphQLResult<RuntimeRecord> {
        records!(self, ctx, Runtime)
    }

    async fn inputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        records!(self, ctx, Input)
    }

    async fn outputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        records!(self, ctx, Output)
    }

    async fn feedback(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        records!(self, ctx, Feedback)
    }

    async fn metadata(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<MetadataRecord>> {
        records!(self, ctx, Metadata)
    }
}

#[ComplexObject]
impl ComponentEventRecord {
    async fn subcomponent_events(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQLResult<Vec<SubcomponentEventRecord>> {
        records!(self, ctx, SubcomponentEventRecord, "subcomponent_event")
    }

    async fn runtime(&self, ctx: &Context<'_>) -> GraphQLResult<RuntimeRecord> {
        records!(self, ctx, Runtime)
    }

    async fn inputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        records!(self, ctx, Input)
    }

    async fn outputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        records!(self, ctx, Output)
    }

    async fn feedback(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        records!(self, ctx, Feedback)
    }

    async fn metadata(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<MetadataRecord>> {
        records!(self, ctx, Metadata)
    }
}

#[ComplexObject]
impl SubcomponentEventRecord {
    async fn runtime(&self, ctx: &Context<'_>) -> GraphQLResult<RuntimeRecord> {
        records!(self, ctx, Runtime)
    }

    async fn inputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        records!(self, ctx, Input)
    }

    async fn outputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        records!(self, ctx, Output)
    }

    async fn feedback(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        records!(self, ctx, Feedback)
    }

    async fn metadata(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<MetadataRecord>> {
        records!(self, ctx, Metadata)
    }
}

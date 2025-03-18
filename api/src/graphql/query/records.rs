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
    ($state:ident, $obj:ident, $conn:ident, Runtime) => {
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
    ($state:ident, $obj:ident, $conn:ident, $io_type:ident, Io) => {
        $crate::unchecked_executor!($state, "io")
            .read_many(async move {
                IORecord::belonging_to($obj)
                    .select(IORecord::as_select())
                    .filter(records_schema::io::deleted_at.is_null())
                    .filter(records_schema::io::io_type.eq(IoTypeEnum::$io_type))
                    .get_results(&mut $conn)
                    .await
                    .map_err(|_| ApiError::GetError)
            })
            .await
            .map_err(|e| e.into())
    };
    ($state:ident, $obj:ident, $conn:ident, Metadata) => {
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
    ($state:ident, $obj:ident, $conn:ident, $event_type:ident, $name:literal, Event) => {
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
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(
            state,
            self,
            conn,
            SubsystemEventRecord,
            "subsystem_event",
            Event
        )
    }

    async fn runtime(&self, ctx: &Context<'_>) -> GraphQLResult<RuntimeRecord> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Runtime)
    }

    async fn inputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Input, Io)
    }

    async fn outputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Output, Io)
    }

    async fn feedback(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Feedback, Io)
    }

    async fn metadata(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<MetadataRecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Metadata)
    }
}

#[ComplexObject]
impl SubsystemEventRecord {
    async fn component_events(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQLResult<Vec<ComponentEventRecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(
            state,
            self,
            conn,
            ComponentEventRecord,
            "component_event",
            Event
        )
    }

    async fn runtime(&self, ctx: &Context<'_>) -> GraphQLResult<RuntimeRecord> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Runtime)
    }

    async fn inputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Input, Io)
    }

    async fn outputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Output, Io)
    }

    async fn feedback(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Feedback, Io)
    }

    async fn metadata(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<MetadataRecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Metadata)
    }
}

#[ComplexObject]
impl ComponentEventRecord {
    async fn subcomponent_events(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQLResult<Vec<SubcomponentEventRecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(
            state,
            self,
            conn,
            SubcomponentEventRecord,
            "subcomponent_event",
            Event
        )
    }

    async fn runtime(&self, ctx: &Context<'_>) -> GraphQLResult<RuntimeRecord> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Runtime)
    }

    async fn inputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Input, Io)
    }

    async fn outputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Output, Io)
    }

    async fn feedback(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Feedback, Io)
    }

    async fn metadata(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<MetadataRecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Metadata)
    }
}

#[ComplexObject]
impl SubcomponentEventRecord {
    async fn runtime(&self, ctx: &Context<'_>) -> GraphQLResult<RuntimeRecord> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Runtime)
    }

    async fn inputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Input, Io)
    }

    async fn outputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Output, Io)
    }

    async fn feedback(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Feedback, Io)
    }

    async fn metadata(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<MetadataRecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        records!(state, self, conn, Metadata)
    }
}

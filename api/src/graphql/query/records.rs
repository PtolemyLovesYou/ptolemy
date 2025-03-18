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
    ($obj:ident, $conn:ident, Runtime) => {
        RuntimeRecord::belonging_to($obj)
            .select(RuntimeRecord::as_select())
            .filter(records_schema::runtime::deleted_at.is_null())
            .get_result(&mut $conn)
            .await
            .map_err(|_| ApiError::GetError.into())
    };
    ($obj:ident, $conn:ident, $io_type:ident, Io) => {
        IORecord::belonging_to($obj)
            .select(IORecord::as_select())
            .filter(records_schema::io::deleted_at.is_null())
            .filter(records_schema::io::io_type.eq(IoTypeEnum::$io_type))
            .get_results(&mut $conn)
            .await
            .map_err(|_| ApiError::GetError.into())
    };
    ($obj:ident, $conn:ident, Metadata) => {
        MetadataRecord::belonging_to($obj)
            .select(MetadataRecord::as_select())
            .filter(records_schema::metadata::deleted_at.is_null())
            .get_results(&mut $conn)
            .await
            .map_err(|_| ApiError::GetError.into())
    };
    ($obj:ident, $conn:ident, $event_type:ident, Event) => {
        $event_type::belonging_to($obj)
            .select($event_type::as_select())
            .get_results(&mut $conn)
            .await
            .map_err(|_| ApiError::GetError.into())
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

        crate::unchecked_executor!(state, "subsystem_event")
            .read_many(async move { records!(self, conn, SubsystemEventRecord, Event) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn runtime(&self, ctx: &Context<'_>) -> GraphQLResult<RuntimeRecord> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "runtime")
            .read(async move { records!(self, conn, Runtime) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn inputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "io")
            .read_many(async move { records!(self, conn, Input, Io) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn outputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "io")
            .read_many(async move { records!(self, conn, Output, Io) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn feedback(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "io")
            .read_many(async move { records!(self, conn, Feedback, Io) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn metadata(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<MetadataRecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "metadata")
            .read_many(async move { records!(self, conn, Metadata) })
            .await
            .map_err(|_| ApiError::GetError.into())
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

        crate::unchecked_executor!(state, "component_event")
            .read_many(async move { records!(self, conn, ComponentEventRecord, Event) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn runtime(&self, ctx: &Context<'_>) -> GraphQLResult<RuntimeRecord> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "runtime")
            .read(async move { records!(self, conn, Runtime) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn inputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "io")
            .read_many(async move { records!(self, conn, Input, Io) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn outputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "io")
            .read_many(async move { records!(self, conn, Output, Io) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn feedback(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "io")
            .read_many(async move { records!(self, conn, Feedback, Io) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn metadata(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<MetadataRecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "metadata")
            .read_many(async move { records!(self, conn, Metadata) })
            .await
            .map_err(|_| ApiError::GetError.into())
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

        crate::unchecked_executor!(state, "subcomponent_event")
            .read_many(async move { records!(self, conn, SubcomponentEventRecord, Event) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn runtime(&self, ctx: &Context<'_>) -> GraphQLResult<RuntimeRecord> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "runtime")
            .read(async move { records!(self, conn, Runtime) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn inputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "io")
            .read_many(async move { records!(self, conn, Input, Io) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn outputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "io")
            .read_many(async move { records!(self, conn, Output, Io) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn feedback(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "io")
            .read_many(async move { records!(self, conn, Feedback, Io) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn metadata(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<MetadataRecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "metadata")
            .read_many(async move { records!(self, conn, Metadata) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }
}

#[ComplexObject]
impl SubcomponentEventRecord {
    async fn runtime(&self, ctx: &Context<'_>) -> GraphQLResult<RuntimeRecord> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "runtime")
            .read(async move { records!(self, conn, Runtime) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn inputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "io")
            .read_many(async move { records!(self, conn, Input, Io) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn outputs(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "io")
            .read_many(async move { records!(self, conn, Output, Io) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn feedback(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<IORecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "io")
            .read_many(async move { records!(self, conn, Feedback, Io) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }

    async fn metadata(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<MetadataRecord>> {
        let state = ctx.data::<GraphQLAppState>()?;

        let mut conn = state.state.get_conn().await?;

        crate::unchecked_executor!(state, "metadata")
            .read_many(async move { records!(self, conn, Metadata) })
            .await
            .map_err(|_| ApiError::GetError.into())
    }
}

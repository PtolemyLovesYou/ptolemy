use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use std::sync::Arc;
use tracing::{error, info};

use api::crud::{
    crypto::verify_password,
    user::{change_user_password, create_user, get_all_users},
};
use api::error::{ApiError, CRUDError};
use api::models::auth::models::UserCreate;
use api::observer::service::MyObserver;
use api::routes::graphql::router::graphql_router;
use api::routes::user::user_router;
use api::routes::workspace::workspace_router;
use api::routes::workspace_user::workspace_user_router;
use api::state::AppState;
use ptolemy_core::generated::observer::observer_server::ObserverServer;
use tokio::try_join;
use tonic::transport::Server;
use tonic_prometheus_layer::metrics::GlobalSettings;

async fn metrics() -> impl IntoResponse {
    match tonic_prometheus_layer::metrics::encode_to_string() {
        Ok(metrics) => metrics.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Creates a base router for the Ptolemy API with default routes.
///
/// This router includes the following routes:
/// - GET `/`: Returns a welcome message indicating that the API is running.
/// - GET `/ping`: Returns a "Pong!" message for a basic health check.
///
/// Returns a `Router` configured with the specified routes.
async fn base_router(enable_prometheus: bool) -> Router {
    let mut router = Router::new()
        .route("/", get(|| async { "Ptolemy API is up and running <3" }))
        .route("/ping", get(|| async { "Pong!" }));

    if enable_prometheus {
        router = router.route("/metrics", get(metrics));
    }

    router
}

async fn ensure_sysadmin(state: &Arc<AppState>) -> Result<(), CRUDError> {
    let mut conn = state.get_conn().await?;

    let user = std::env::var("PTOLEMY_USER").expect("PTOLEMY_USER must be set.");
    let pass = std::env::var("PTOLEMY_PASS").expect("PTOLEMY_PASS must be set.");

    let users_list = get_all_users(&mut conn).await?;

    for user in users_list {
        if user.is_sysadmin {
            if verify_password(&mut conn, &pass, &user.salt, &user.password_hash).await? {
                return Ok(());
            }
            // update password
            else {
                change_user_password(&mut conn, &user.id, &pass).await?;
                return Ok(());
            }
        }
    }

    match create_user(
        &mut conn,
        &UserCreate {
            username: user,
            display_name: Some("SYSADMIN".to_string()),
            is_sysadmin: true,
            is_admin: false,
            password: pass,
        },
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to create sysadmin: {:?}", e);
            Err(CRUDError::InsertError)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    tracing_subscriber::fmt::init();

    let shared_state = Arc::new(AppState::new().await?);

    // ensure sysadmin
    match ensure_sysadmin(&shared_state).await {
        Ok(_) => (),
        Err(err) => {
            error!("Failed to set up sysadmin. This may be because the Postgres db is empty. Run Diesel migrations and then try again. More details: {:?}", err);
        }
    };

    // gRPC server setup
    let grpc_addr = "[::]:50051".parse().unwrap();
    let observer = MyObserver::new(shared_state.clone()).await;

    // Axum server setup
    let app = Router::new()
        .nest("/", base_router(shared_state.enable_prometheus).await)
        .nest("/graphql", graphql_router().await)
        .nest("/workspace", workspace_router(&shared_state).await)
        .nest("/user", user_router(&shared_state).await)
        .nest(
            "/workspace_user",
            workspace_user_router(&shared_state).await,
        );

    let server_url = format!("0.0.0.0:{}", shared_state.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    info!("Observer server listening on {}", grpc_addr);
    info!("Axum server serving at {}", &server_url);

    // Run both servers concurrently
    if shared_state.enable_prometheus {
        info!("Prometheus metrics enabled");
        tonic_prometheus_layer::metrics::try_init_settings(GlobalSettings {
            histogram_buckets: vec![0.01, 0.05, 0.1, 0.5, 1.0, 2.5, 5.0, 10.0],
            ..Default::default()
        })
        .unwrap();

        let metrics_layer = tonic_prometheus_layer::MetricsLayer::new();

        try_join!(
            // gRPC server with metrics
            async move {
                match Server::builder()
                    .layer(metrics_layer)
                    .add_service(ObserverServer::new(observer))
                    .serve(grpc_addr)
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        info!("gRPC server error: {}", e);
                        return Err(ApiError::APIError);
                    }
                }
            },
            // Axum server
            async move {
                match axum::serve(listener, app).await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        info!("Axum server error: {}", e);
                        return Err(ApiError::GRPCError);
                    }
                }
            }
        )?;
    } else {
        try_join!(
            // gRPC server without metrics
            async move {
                match Server::builder()
                    .add_service(ObserverServer::new(observer))
                    .serve(grpc_addr)
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        info!("gRPC server error: {}", e);
                        return Err(ApiError::APIError);
                    }
                }
            },
            // Axum server
            async move {
                match axum::serve(listener, app).await {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        info!("Axum server error: {}", e);
                        return Err(ApiError::GRPCError);
                    }
                }
            }
        )?;
    }

    Ok(())
}

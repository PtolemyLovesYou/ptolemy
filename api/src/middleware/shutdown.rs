use crate::state::ApiAppState;
use tokio::signal;

pub async fn shutdown_signal(state: ApiAppState) {
    let ctrl_c = async {
        signal::ctrl_c().await.unwrap();
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap()
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::error!("Shutting down");
            state.shutdown().await.ok();
        },
        _ = terminate => {
            tracing::error!("Shutting down");
            state.shutdown().await.ok();
        },
    }
}

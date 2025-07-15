use super::{sink::Sink, state::PtolemyState};
use tokio::signal;

pub async fn shutdown_signal(_state: PtolemyState, sink: Sink) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::error!("🛑 Received Ctrl+C, shutting down...");
        },
        _ = terminate => {
            tracing::error!("🛑 Received SIGTERM, shutting down...");
        },
    }

    // Trigger sink shutdown by dropping the sender
    drop(sink);
}

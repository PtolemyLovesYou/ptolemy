use super::{sink::SinkMessage, state::PtolemyState};
use tokio::{
    signal,
    task::JoinHandle,
    time::{timeout, Duration},
};

pub async fn shutdown_signal(state: PtolemyState, sink_join_handle: JoinHandle<()>) {
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

    // Send shutdown signal to sink
    let tx = state.sender();
    if let Err(e) = tx.send(SinkMessage::Shutdown).await {
        tracing::error!("Error shutting down sink consumer: {}", e);
    };

    drop(tx);

    // Trigger sink shutdown by dropping the sender
    match timeout(Duration::from_secs(30), sink_join_handle).await {
        Ok(join_result) => match join_result {
            Ok(_) => tracing::debug!("Successfully wrote all messages."),
            Err(e) => tracing::error!("Sink cancelled or panicked: {}", e),
        },
        Err(e) => tracing::error!("Timeout elapsed before task was finished: {}", e),
    }
}

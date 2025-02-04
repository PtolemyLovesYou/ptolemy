use tower_http::classify::{
    // GrpcErrorsAsFailures,
    ServerErrorsAsFailures
};
use tower_http::{
    trace::{self, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

type TraceMiddleware<T> = TraceLayer<tower_http::classify::SharedClassifier<T>>;

pub fn trace_layer<T>(layer: TraceMiddleware<T>) -> TraceMiddleware<T> {
    layer
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            trace::DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Micros),
        )
        .on_body_chunk(trace::DefaultOnBodyChunk::new())
        .on_eos(
            trace::DefaultOnEos::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Micros),
        )
        .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR))
}

pub fn trace_layer_rest() -> TraceMiddleware<ServerErrorsAsFailures> {
    trace_layer(TraceLayer::new_for_http())
}

// pub fn trace_layer_grpc() -> TraceMiddleware<GrpcErrorsAsFailures> {
//     trace_layer(TraceLayer::new_for_grpc())
// }

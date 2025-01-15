use tower_http::{
    trace::{self, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

pub mod auth;
pub mod request_context;

type HttpTraceLayer = TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
>;

pub fn trace_layer_rest() -> HttpTraceLayer {
    TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
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

type GrpcTraceLayer =
    TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::GrpcErrorsAsFailures>>;

pub fn trace_layer_grpc() -> GrpcTraceLayer {
    TraceLayer::new_for_grpc()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
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

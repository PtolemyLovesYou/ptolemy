#[macro_export]
macro_rules! trace_layer {
    (Http) => { $crate::trace_layer!(new_for_http) };
    // (Grpc) => { crate::trace_layer!(new_for_grpc) };
    ($type:ident) => {
        tower_http::trace::TraceLayer::$type()
            .on_request(|request: &axum::http::Request<_>, _: &_| {
                tracing::info!(
                    method = %request.method(),
                    path = %request.uri().path(),
                    "incoming request"
                );
            })
            .on_response(
                tower_http::trace::DefaultOnResponse::new()
                    .level(tracing::Level::INFO)
                    .latency_unit(tower_http::LatencyUnit::Micros),
            )
            .on_body_chunk(tower_http::trace::DefaultOnBodyChunk::new())
            .on_eos(
                tower_http::trace::DefaultOnEos::new()
                    .level(tracing::Level::INFO)
                    .latency_unit(tower_http::LatencyUnit::Micros),
            )
            .on_failure(tower_http::trace::DefaultOnFailure::new().level(tracing::Level::ERROR))
    };
}

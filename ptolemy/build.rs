fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build protobufs and write to src/generated
    // Only happens if env var BUILD_PROTOBUFS is set to 1
    let build_protobufs = std::env::var("BUILD_PROTOBUFS")
        .map(|s| s == "1")
        .unwrap_or(false);

    if build_protobufs {
        tonic_build::configure()
            .protoc_arg("--experimental_allow_proto3_optional")
            .build_server(true)
            .out_dir("src/generated")
            .compile_protos(
                &["proto/record_publisher.proto"],
                &["proto/record_publisher"],
            )?;
    }

    Ok(())
}

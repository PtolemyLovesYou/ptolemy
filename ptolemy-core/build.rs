fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_protobufs = std::env::var("BUILD_PROTOBUFS")
        .map(|s| s == "1")
        .unwrap_or(true);

    if build_protobufs {
        tonic_build::configure()
            .build_server(true)
            .out_dir("src/generated")
            .compile_protos(&["proto/observer.proto"], &["proto/observer"])?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
         .build_server(false)
         .out_dir("src/generated")
         .compile_protos(
             &["proto/observer.proto"],
             &["proto/observer"],
         )?;
    Ok(())
}

use api::{config::PtolemyConfig, error::ApiError};

fn main() -> Result<(), ApiError> {
    println!("{:?}", PtolemyConfig::from_file()?);
    Ok(())
}

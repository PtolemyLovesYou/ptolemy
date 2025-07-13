#[derive(Debug)]
pub enum PtolemyError {
    ServerError,
    UndefinedTier,
    InvalidUuid,
    MissingData,
    InvalidJson,
    InvalidTimestamp,
}

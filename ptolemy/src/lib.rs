use crate::client::client::PtolemyClient;
use pyo3::prelude::*;
use crate::models::enums::{ApiKeyPermission, UserStatus, WorkspaceRole};

pub mod client;
pub mod config;
pub mod event;
pub mod types;
pub mod models;
pub mod utils;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PtolemyClient>()?;
    m.add_class::<ApiKeyPermission>()?;
    m.add_class::<UserStatus>()?;
    m.add_class::<WorkspaceRole>()?;
    Ok(())
}

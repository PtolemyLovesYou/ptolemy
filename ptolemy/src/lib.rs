#[cfg(feature = "client")]
use crate::{
    client::client::PtolemyClient,
    models::enums::{api_key_permission_enum, user_status_enum, workspace_role_enum}
};
#[cfg(feature = "client")]
use pyo3::prelude::*;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "client")]
pub mod config;
#[cfg(feature = "client")]
pub mod types;
#[cfg(feature = "client")]
pub mod models;
#[cfg(feature = "client")]
pub mod utils;

pub mod generated;
#[cfg(feature = "client")]
pub mod event;
pub mod parser;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[cfg(feature = "client")]
#[pymodule]
fn _core<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
    m.add_class::<PtolemyClient>()?;
    api_key_permission_enum::add_enum_to_module(py, m)?;
    user_status_enum::add_enum_to_module(py, m)?;
    workspace_role_enum::add_enum_to_module(py, m)?;
    Ok(())
}

pub mod generated;
pub mod parser;

#[cfg(feature = "python")]
pub mod client;
#[cfg(feature = "python")]
pub mod config;
#[cfg(feature = "python")]
pub mod types;
#[cfg(feature = "python")]
pub mod models;
#[cfg(feature = "python")]
pub mod utils;
#[cfg(feature = "python")]
pub mod event;

#[cfg(feature = "python")]
mod python {
    use pyo3::prelude::*;

    use crate::{
        client::client::PtolemyClient,
        models::enums::{api_key_permission_enum, user_status_enum, workspace_role_enum}
    };
    /// A Python module implemented in Rust. The name of this function must match
    /// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
    /// import the module.
    #[pymodule]
    pub fn _core<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
        m.add_class::<PtolemyClient>()?;
        api_key_permission_enum::add_enum_to_module(py, m)?;
        user_status_enum::add_enum_to_module(py, m)?;
        workspace_role_enum::add_enum_to_module(py, m)?;
        Ok(())
    }
}

#[cfg(feature = "python")]
pub use python::_core;

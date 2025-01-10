use pyo3::prelude::*;

pub mod client;
pub mod enums;
pub mod models;
pub mod types;
pub mod graphql;

use crate::{
    pybindings::client::client::PtolemyClient,
    pybindings::{
        enums::{api_key_permission, user_status, workspace_role},
        models::add_models_to_module,
        graphql::PyGraphQLClient,
    },
};
/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
pub fn _core<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
    m.add_class::<PtolemyClient>()?;
    m.add_class::<PyGraphQLClient>()?;
    add_models_to_module(py, m)?;
    api_key_permission::add_enum_to_module(py, m)?;
    user_status::add_enum_to_module(py, m)?;
    workspace_role::add_enum_to_module(py, m)?;
    Ok(())
}

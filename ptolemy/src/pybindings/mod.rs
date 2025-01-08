use pyo3::prelude::*;

pub mod enums;
pub mod models;
pub mod types;
pub mod event;

use crate::{
    client::client::PtolemyClient,
    pybindings::{
        enums::{api_key_permission, workspace_role, user_status,},
        models::add_models_to_module,
    },
};
/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
pub fn _core<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
    m.add_class::<PtolemyClient>()?;
    add_models_to_module(py, m)?;
    api_key_permission::add_enum_to_module(py, m)?;
    user_status::add_enum_to_module(py, m)?;
    workspace_role::add_enum_to_module(py, m)?;
    Ok(())
}

use crate::client::client::PtolemyClient;
use pyo3::prelude::*;
use crate::models::enums::{api_key_permission_enum, user_status_enum, workspace_role_enum};

pub mod client;
pub mod config;
pub mod event;
pub mod types;
pub mod models;
pub mod utils;

#[derive(Clone, Debug)]
pub struct MyStruct {
    foo: String,
    bar: String,
}

pymodel!(MyStruct, MyStructWrapper, [foo, bar]);

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
    m.add_class::<PtolemyClient>()?;
    api_key_permission_enum::add_enum_to_module(py, m)?;
    user_status_enum::add_enum_to_module(py, m)?;
    workspace_role_enum::add_enum_to_module(py, m)?;
    Ok(())
}

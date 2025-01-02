use crate::client::PtolemyClient;
use pyo3::prelude::*;

pub mod client;
pub mod config;
pub mod event;
pub mod types;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PtolemyClient>()?;
    Ok(())
}

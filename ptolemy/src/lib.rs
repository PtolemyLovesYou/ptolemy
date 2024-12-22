use crate::publish::BlockingObserverClient;
use crate::event::{Event, Runtime, IO, Metadata};
use pyo3::prelude::*;

pub mod publish;
pub mod config;
pub mod event;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BlockingObserverClient>()?;
    m.add_class::<Event>()?;
    m.add_class::<Runtime>()?;
    m.add_class::<IO>()?;
    m.add_class::<Metadata>()?;
    Ok(())
}

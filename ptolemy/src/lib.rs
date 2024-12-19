use crate::publish::{BlockingObserverClient, ProtoRecord, RecordBuilder};
use pyo3::prelude::*;

pub mod publish;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BlockingObserverClient>()?;
    m.add_class::<ProtoRecord>()?;
    m.add_class::<RecordBuilder>()?;
    Ok(())
}

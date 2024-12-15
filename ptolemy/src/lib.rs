use pyo3::prelude::*;
use crate::publish::{
    BlockingObserverClient,
    RecordBuilder,
    ProtoRecord
};

pub mod publish;
pub mod generated;

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

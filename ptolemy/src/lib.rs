use pyo3::prelude::*;
use crate::publish::{
    {ClientConfig::ObserverConfig, BlockingObserverClient},
    RecordBuilder,
    ProtoRecord
};

pub mod publish;

#[pyfunction]
fn hello_from_bin() -> String {
    let client = BlockingObserverClient::connect(ObserverConfig::new());
    "Hello from ptolemy!".to_string()
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    m.add_class::<BlockingObserverClient>()?;
    m.add_class::<ProtoRecord>()?;
    m.add_class::<RecordBuilder>()?;
    Ok(())
}

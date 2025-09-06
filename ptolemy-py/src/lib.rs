use pyo3::prelude::*;

mod runtime;

pub mod types;
pub mod v1;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
pub fn _core<'a>(_py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
    m.add_class::<v1::RecordExporter>()?;
    m.add_function(wrap_pyfunction!(v1::validate_field_value, m)?)?;
    Ok(())
}

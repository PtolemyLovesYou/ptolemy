use pyo3::prelude::*;
use pyo3::types::{PyType, PyTraceback};
use pyo3::exceptions::PyBaseException;

pub type ExcType<'a> = Bound<'a, PyType>;
pub type ExcValue<'a> = Bound<'a, PyBaseException>;
pub type Traceback<'a> = Bound<'a, PyTraceback>;

pub fn format_traceback(exc_type: ExcType<'_>, exc_value: ExcValue<'_>, traceback: Traceback<'_>) -> PyResult<String> {
    Python::with_gil(|py| {
        let traceback_module = py.import_bound("traceback")?;
        let format_result = traceback_module
            .getattr("format_exception")?
            .call1((exc_type, exc_value, traceback));
            
        match format_result {
            Ok(result) => result.extract(),
            Err(e) => Ok(format!("Error formatting traceback: {}", e))
        }
    })
}

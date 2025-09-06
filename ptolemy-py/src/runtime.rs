use once_cell::sync::OnceCell;
use pyo3::{exceptions::PyRuntimeError, PyResult};
use tokio::runtime::Runtime;

static RUNTIME: OnceCell<Runtime> = OnceCell::new();

pub fn runtime() -> PyResult<&'static Runtime> {
    RUNTIME.get_or_try_init(|| {
        Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create Tokio runtime: {e}")))
    })
}

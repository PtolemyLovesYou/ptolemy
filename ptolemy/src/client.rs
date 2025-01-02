use uuid::Uuid;
use pyo3::prelude::*;
use ptolemy_core::generated::observer::Tier;

#[derive(Clone, Debug)]
#[pyclass]
pub struct PtolemyClient {
    workspace_id: Uuid,
    tier: Tier,
    autoflush: bool,
}

#[pymethods]
impl PtolemyClient {
}

use ptolemy_core::generated::observer::Tier;
use pyo3::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug)]
#[pyclass]
pub struct PtolemyClient {
    workspace_id: Uuid,
    tier: Tier,
    autoflush: bool,
}

#[pymethods]
impl PtolemyClient {}

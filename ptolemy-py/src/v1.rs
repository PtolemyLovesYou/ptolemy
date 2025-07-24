use super::types::{PyUUIDWrapper, PyJSON};
use pyo3::prelude::*;

#[derive(Debug, FromPyObject)]
pub struct IO {
    pub parent_id: PyUUIDWrapper,
    #[pyo3(attribute("id_"))]
    pub id: PyUUIDWrapper,

    pub field_name: String,
    pub field_value: PyJSON,
}

#[derive(Debug, FromPyObject)]
pub struct Metadata {
    pub parent_id: PyUUIDWrapper,
    #[pyo3(attribute("id_"))]
    pub id: PyUUIDWrapper,

    pub field_name: String,
    pub field_value: String,
}

#[derive(Debug, FromPyObject)]
pub struct Trace {
    pub parent_id: PyUUIDWrapper,
    #[pyo3(attribute("id_"))]
    pub id: PyUUIDWrapper,

    pub name: String,
    pub parameters: Option<PyJSON>,

    pub start_time: Option<f32>,
    pub end_time: Option<f32>,
    pub error_type: Option<String>,
    pub error_content: Option<String>,

    #[pyo3(attribute("inputs_"))]
    pub inputs: Option<Vec<IO>>,
    #[pyo3(attribute("outputs_"))]
    pub outputs: Option<Vec<IO>>,
    #[pyo3(attribute("feedback_"))]
    pub feedback: Option<Vec<IO>>,
    #[pyo3(attribute("metadata_"))]
    pub metadata: Option<Vec<Metadata>>,
}

#[derive(Debug)]
#[pyclass]
pub struct RecordExporter;

#[pymethods]
impl RecordExporter {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_trace(&self, trace: Trace) -> PyResult<()> {
        println!("name: {}", trace.name);

        Ok(())
    }
}

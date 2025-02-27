// use crate::generated::observer;
use ptolemy::models::{Id, JSON};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use std::str::FromStr;
use uuid::Uuid;

#[derive(FromPyObject)]
pub struct PyUUIDWrapper {
    hex: String,
}

impl Into<Uuid> for PyUUIDWrapper {
    fn into(self) -> Uuid {
        Uuid::from_str(&self.hex).unwrap()
    }
}

#[derive(FromPyObject)]
pub enum PyId {
    UUID(PyUUIDWrapper),
    String(String),
}

impl Into<PyId> for Id {
    fn into(self) -> PyId {
        PyId::UUID(PyUUIDWrapper {
            hex: self.to_string(),
        })
    }
}

impl<'py> IntoPyObject<'py> for PyId {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        let uuid = py.import("uuid")?.getattr("UUID")?;

        let hex = match self {
            PyId::UUID(u) => u.hex,
            PyId::String(s) => s,
        };

        uuid.call1((hex,))
    }
}

impl Into<Id> for PyId {
    fn into(self) -> Id {
        match self {
            PyId::UUID(u) => Id::from(Uuid::from_str(&u.hex).unwrap()),
            PyId::String(s) => Id::from(Uuid::from_str(&s).unwrap()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PyJSON(pub JSON);

impl From<JSON> for PyJSON {
    fn from(json: JSON) -> Self {
        PyJSON(json)
    }
}

impl From<PyJSON> for JSON {
    fn from(pyjson: PyJSON) -> Self {
        pyjson.0
    }
}

impl<'py> FromPyObject<'py> for PyJSON {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<PyJSON> {
        if let Ok(s) = obj.downcast::<PyString>() {
            Ok(PyJSON(JSON(json!(s.extract::<String>()?))))
        } else if let Ok(i) = obj.downcast::<PyInt>() {
            Ok(PyJSON(JSON(json!(i.extract::<i32>()?))))
        } else if let Ok(f) = obj.downcast::<PyFloat>() {
            Ok(PyJSON(JSON(json!(f.extract::<f32>()?))))
        } else if let Ok(b) = obj.downcast::<PyBool>() {
            Ok(PyJSON(JSON(json!(b.extract::<bool>()?))))
        } else if let Ok(d) = obj.downcast::<PyDict>() {
            let mut inner = BTreeMap::new();
            for (k, v) in d.iter() {
                inner.insert(k.extract::<String>()?, v.extract::<PyJSON>()?);
            }
            Ok(PyJSON(JSON(json!((inner)))))
        } else if let Ok(l) = obj.downcast::<PyList>() {
            let mut inner = Vec::new();
            for v in l.iter() {
                inner.push(v.extract::<PyJSON>()?);
            }
            Ok(PyJSON(JSON(json!(inner))))
        } else {
            Err(PyValueError::new_err(format!(
                "Unsupported type: {}",
                obj.get_type().name()?.extract::<String>()?
            )))
        }
    }
}

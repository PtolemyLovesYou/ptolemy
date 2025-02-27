// use crate::generated::observer;
use crate::models::{Id, JSON};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString};
use std::collections::BTreeMap;
use std::str::FromStr;
use uuid::Uuid;
use serde_json::json;

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

impl Into<Id> for PyId {
    fn into(self) -> Id {
        match self {
            PyId::UUID(u) => Id::from(Uuid::from_str(&u.hex).unwrap()),
            PyId::String(s) => Id::from(Uuid::from_str(&s).unwrap()),
        }
    }
}

impl<'py> FromPyObject<'py> for Id {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Id> {
        let uuid = obj
            .extract::<PyId>()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(uuid.into())
    }
}

impl<'py> IntoPyObject<'py> for Id {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let import = py.import("uuid").unwrap();
        let uuid_type = import.getattr("UUID").unwrap();
        Ok(uuid_type.call1((self.to_string(),)).unwrap())
    }
}

impl<'py> FromPyObject<'py> for JSON {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<JSON> {
        if let Ok(s) = obj.downcast::<PyString>() {
            Ok(JSON(json!(s.extract::<String>()?)))
        } else if let Ok(i) = obj.downcast::<PyInt>() {
            Ok(JSON(json!(i.extract::<i32>()?)))
        } else if let Ok(f) = obj.downcast::<PyFloat>() {
            Ok(JSON(json!(f.extract::<f32>()?)))
        } else if let Ok(b) = obj.downcast::<PyBool>() {
            Ok(JSON(json!(b.extract::<bool>()?)))
        } else if let Ok(d) = obj.downcast::<PyDict>() {
            let mut inner = BTreeMap::new();
            for (k, v) in d.iter() {
                inner.insert(k.extract::<String>()?, v.extract::<JSON>()?);
            }
            Ok(JSON(json!((inner))))
        } else if let Ok(l) = obj.downcast::<PyList>() {
            let mut inner = Vec::new();
            for v in l.iter() {
                inner.push(v.extract::<JSON>()?);
            }
            Ok(JSON(json!(inner)))
        } else {
            Err(PyValueError::new_err(format!(
                "Unsupported type: {}",
                obj.get_type().name()?.extract::<String>()?
            )))
        }
    }
}

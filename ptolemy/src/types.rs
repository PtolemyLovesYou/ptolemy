// use crate::generated::observer;
use prost_types::value::Kind;
use prost_types::{ListValue, Struct, Value};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(FromPyObject)]
pub struct PyUuid {
    hex: String,
}

impl TryFrom<PyUuid> for Uuid {
    type Error = PyErr;

    fn try_from(uuid: PyUuid) -> Result<Uuid, Self::Error> {
        Uuid::parse_str(&uuid.hex).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

impl TryFrom<Uuid> for PyUuid {
    type Error = PyErr;

    fn try_from(uuid: Uuid) -> Result<PyUuid, Self::Error> {
        Ok(PyUuid { hex: uuid.to_string() })
    }
}

impl<'py> IntoPyObject<'py> for PyUuid {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let import = py.import("uuid").unwrap();
        let uuid_type = import.getattr("UUID").unwrap();
        Ok(uuid_type.call1((self.hex,)).unwrap())
    }
}

pub fn get_uuid(id: &str) -> PyResult<Uuid> {
    match Uuid::parse_str(&id) {
        Ok(i) => Ok(i),
        Err(e) => {
            let error_msg = format!("Unable to parse UUID: {}", e);
            Err(PyValueError::new_err(error_msg))
        }
    }
}

#[derive(FromPyObject, Clone, Debug)]
pub enum JsonSerializable {
    String(String),
    Int(isize),
    Float(f64),
    Bool(bool),
    Dict(BTreeMap<String, Option<JsonSerializable>>),
    List(Vec<Option<JsonSerializable>>),
}

#[derive(FromPyObject, Clone, Debug)]
#[pyo3(transparent)]
pub struct Parameters {
    inner: BTreeMap<String, JsonSerializable>,
}

impl Parameters {
    pub fn into_inner(self) -> BTreeMap<String, JsonSerializable> {
        self.inner
    }
}

pub fn parameters_to_value(params: &Parameters) -> Option<Value> {
    let mut fields = BTreeMap::new();
    for (k, v) in &params.inner {
        if let Some(value) = json_serializable_to_value(&Some(v.clone())) {
            fields.insert(k.clone(), value);
        }
    }
    Some(Value {
        kind: Some(Kind::StructValue(Struct { fields })),
    })
}

pub fn json_serializable_to_value(json: &Option<JsonSerializable>) -> Option<Value> {
    match json {
        None => None,
        Some(JsonSerializable::String(s)) => Some(Value {
            kind: Some(Kind::StringValue(s.clone())),
        }),
        Some(JsonSerializable::Int(i)) => Some(Value {
            kind: Some(Kind::NumberValue(*i as f64)),
        }),
        Some(JsonSerializable::Float(f)) => Some(Value {
            kind: Some(Kind::NumberValue(*f)),
        }),
        Some(JsonSerializable::Bool(b)) => Some(Value {
            kind: Some(Kind::BoolValue(*b)),
        }),
        Some(JsonSerializable::Dict(d)) => {
            let mut fields = BTreeMap::new();
            for (k, v) in d {
                if let Some(value) = json_serializable_to_value(v) {
                    fields.insert(k.clone(), value);
                }
            }
            Some(Value {
                kind: Some(Kind::StructValue(Struct { fields })),
            })
        }
        Some(JsonSerializable::List(l)) => {
            let values: Vec<Value> = l
                .iter()
                .filter_map(|v| json_serializable_to_value(v))
                .collect();
            Some(Value {
                kind: Some(Kind::ListValue(ListValue { values })),
            })
        }
    }
}

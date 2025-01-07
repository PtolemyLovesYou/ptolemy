use crate::client::client::PtolemyClient;
use pyo3::{exceptions::PyValueError, prelude::*};
use crate::models::enums::{ApiKeyPermission, UserStatus, WorkspaceRole};
use pyo3_ffi::c_str;
use std::ffi::CStr;
use once_cell::sync::Lazy;
use pyo3::types::PyType;

pub mod client;
pub mod config;
pub mod event;
pub mod types;
pub mod models;
pub mod utils;

fn generate_from_python(pystr: &CStr, module_name: &CStr, filename: &CStr, class_name: &str) -> PyResult<Py<PyType>> {
    Python::with_gil(|py| {
        let enum_type = PyModule::from_code(
            py,
            pystr,
            filename,
            module_name,
        )?
        .getattr(class_name)?
        .downcast::<PyType>()?
        .clone()
        .unbind();

        Ok(enum_type)
    })
}

const MY_STRENUM: &CStr = c_str!(r###"
from enum import StrEnum

class MyStrEnum(StrEnum):
    ENUM1 = "enum_1"
    ENUM2 = "enum_2"
    ENUM3 = "enum_3"
"###);

static ENUM_CLS: Lazy<Py<PyType>> = Lazy::new(|| {
    match generate_from_python(
        MY_STRENUM,
        c_str!("my_strenum"),
        c_str!("my_strenum.py"),
        "MyStrEnum"
    ) {
        Ok(enum_type) => enum_type,
        Err(e) => panic!("Failed to generate MyStrEnum: {}", e)
    }
});

#[derive(Clone, Debug, PartialEq)]
enum MyEnum {
    Enum1,
    Enum2,
    Enum3
}

impl Into<String> for MyEnum {
    fn into(self) -> String {
        match self {
            MyEnum::Enum1 => String::from("enum_1"),
            MyEnum::Enum2 => String::from("enum_2"),
            MyEnum::Enum3 => String::from("enum_3")
        }
    }
}

impl TryFrom<String> for MyEnum {
    type Error = PyErr;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "enum_1" => Ok(MyEnum::Enum1),
            "enum_2" => Ok(MyEnum::Enum2),
            "enum_3" => Ok(MyEnum::Enum3),
            _ => Err(PyValueError::new_err(format!("Invalid enum value: {}", value)))
        }
    }
}

impl <'py> FromPyObject<'py> for MyEnum {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        MyEnum::try_from(ob.extract::<String>()?)
    }
}

impl<'py> IntoPyObject<'py> for MyEnum {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = std::convert::Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: String = self.into();
        let val = ENUM_CLS.bind(py).call1((s,)).unwrap();
        Ok(val)
    }
}

fn add_enum_to_module<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
    m.add("MyStrEnum", ENUM_CLS.bind(py))?;
    Ok(())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
    add_enum_to_module(py, m)?;
    m.add_class::<PtolemyClient>()?;
    m.add_class::<ApiKeyPermission>()?;
    m.add_class::<UserStatus>()?;
    m.add_class::<WorkspaceRole>()?;
    Ok(())
}

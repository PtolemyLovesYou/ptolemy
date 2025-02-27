use std::ffi::CStr;

use crate::models::{ServiceApiKey, User, UserApiKey, Workspace, WorkspaceUser};
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};

static MODEL_FORMATTER: &CStr =
    c_str!(r#"'{}({})'.format(name, ', '.join(k + '=' + repr(v) for k, v in model_attrs))"#);

macro_rules! pymodel {
    // if name isn't specified in the $meta macro, no guarantees that the Python class name will be correct
    ($struct:ty, $name:ident, [$($meta:tt)*], [$($getter:ident),+ $(,)?]) => {
        #[pyclass(frozen, $($meta)*)]
        #[derive(Clone, Debug)]
        pub struct $name($struct);

        #[pymethods]
        impl $name {
            $(
                #[getter]
                fn $getter(&self) -> PyResult<impl IntoPyObject> {
                    Ok(self.0.$getter.clone())
                }
            )+

            pub fn __dict__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
                let dict = PyDict::new(py);

                $(
                    dict.set_item(stringify!($getter), self.$getter()?)?;
                )+

                Ok(dict)
            }

            pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
                self.__dict__(py)
            }

            pub fn __repr__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                let attrs: Bound<'_, PyList> = PyList::empty(py);

                $(
                    attrs.append(
                        (
                            stringify!($getter),
                            self.$getter()?
                        )
                    )?;
                )+

                let data: Bound<'_, PyDict> = PyDict::new(py);
                data.set_item("model_attrs", attrs)?;
                data.set_item("name", stringify!($struct))?;

                let repr = py.eval(
                    MODEL_FORMATTER,
                    None,
                    Some(&data)
                )?;

                Ok(repr)
            }
        }

        impl From<$struct> for $name {
            fn from(value: $struct) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $struct {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl<'py> IntoPyObject<'py> for $struct {
            type Target = $name;
            type Output = Bound<'py, Self::Target>;
            type Error = PyErr;

            fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
                Bound::new(py, $name(self))
            }
        }

        impl<'py> FromPyObject<'py> for $struct {
            fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
                Ok(ob.extract::<$name>()?.into())
            }
        }
    }
}

pymodel!(
    Workspace,
    PyWorkspace,
    [name = "Workspace"],
    [id, name, description, archived, created_at, updated_at]
);
pymodel!(
    User,
    PyUser,
    [name = "User"],
    [id, username, display_name, status, is_admin, is_sysadmin]
);
pymodel!(
    UserApiKey,
    PyUserApiKey,
    [name = "UserApiKey"],
    [id, user_id, name, key_preview, expires_at]
);
pymodel!(
    ServiceApiKey,
    PyServiceApiKey,
    [name = "ServiceApiKey"],
    [id, workspace_id, name, key_preview, expires_at, permissions]
);
pymodel!(
    WorkspaceUser,
    PyWorkspaceUser,
    [name = "WorkspaceUser"],
    [workspace_id, user_id, role]
);

pub fn add_models_to_module<'a>(_py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
    m.add_class::<PyWorkspace>()?;
    m.add_class::<PyUser>()?;
    m.add_class::<PyUserApiKey>()?;
    m.add_class::<PyServiceApiKey>()?;
    m.add_class::<PyWorkspaceUser>()?;
    Ok(())
}

use std::ffi::CStr;

// use ptolemy::models::{ServiceApiKey, User, UserApiKey, Workspace, WorkspaceUser};
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};

static MODEL_FORMATTER: &CStr =
    c_str!(r#"'{}({})'.format(name, ', '.join(k + '=' + repr(v) for k, v in model_attrs))"#);

macro_rules! pymodel {
    // if name isn't specified in the $meta macro, no guarantees that the Python class name will be correct
    ($struct:ident, $name:ident, [$($meta:tt)*], [$(($getter:ident, $gty:ty)),+ $(,)?]) => {
        #[pyclass(frozen, $($meta)*)]
        #[derive(Clone, Debug)]
        pub struct $name(ptolemy::models::$struct);

        #[pymethods]
        impl $name {
            $(
                #[getter]
                fn $getter(&self) -> PyResult<$gty> {
                    Ok(self.0.$getter.clone().into())
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

        impl From<ptolemy::models::$struct> for $name {
            fn from(value: ptolemy::models::$struct) -> Self {
                Self(value)
            }
        }

        impl From<$name> for ptolemy::models::$struct {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        // impl<'py> IntoPyObject<'py> for $struct {
        //     type Target = $name;
        //     type Output = Bound<'py, Self::Target>;
        //     type Error = PyErr;

        //     fn into_pyobject(self, py: Python<'py>) -> PyResult<Self::Output> {
        //         Bound::new(py, $name(self))
        //     }
        // }

        // impl<'py> FromPyObject<'py> for $struct {
        //     fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        //         Ok(ob.extract::<$name>()?.into())
        //     }
        // }
    }
}

pymodel!(
    Workspace,
    PyWorkspace,
    [name = "Workspace"],
    [(id, crate::types::PyId), (name, String), (description, Option<String>), (archived, bool), (created_at, chrono::DateTime<chrono::Utc>), (updated_at, chrono::DateTime<chrono::Utc>)]
);
pymodel!(
    User,
    PyUser,
    [name = "User"],
    [(id, crate::types::PyId), (username, String), (display_name, Option<String>), (status, crate::enums::user_status::UserStatus), (is_admin, bool), (is_sysadmin, bool)]
);
pymodel!(
    UserApiKey,
    PyUserApiKey,
    [name = "UserApiKey"],
    [(id, crate::types::PyId), (user_id, crate::types::PyId), (name, String), (key_preview, String), (expires_at, Option<chrono::DateTime<chrono::Utc>>)]
);
pymodel!(
    ServiceApiKey,
    PyServiceApiKey,
    [name = "ServiceApiKey"],
    [(id, crate::types::PyId), (workspace_id, crate::types::PyId), (name, String), (key_preview, String), (expires_at, Option<chrono::DateTime<chrono::Utc>>), (permissions, crate::enums::api_key_permission::ApiKeyPermission)]
);
pymodel!(
    WorkspaceUser,
    PyWorkspaceUser,
    [name = "WorkspaceUser"],
    [
        (workspace_id, crate::types::PyId),
        (user_id, crate::types::PyId),
        (role, crate::enums::workspace_role::WorkspaceRole)
    ]
);

pub fn add_models_to_module<'a>(_py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
    m.add_class::<PyWorkspace>()?;
    m.add_class::<PyUser>()?;
    m.add_class::<PyUserApiKey>()?;
    m.add_class::<PyServiceApiKey>()?;
    m.add_class::<PyWorkspaceUser>()?;
    Ok(())
}

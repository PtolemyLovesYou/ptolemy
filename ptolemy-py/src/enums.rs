use pyo3::prelude::*;
use pyo3::sync::GILOnceCell;
use pyo3::types::PyType;

pub trait PyEnumCompatible<'py, 'de>:
    IntoPyObject<'py, Target = PyAny, Output = Bound<'py, PyAny>, Error = PyErr>
    + FromPyObject<'py>
    + Clone
    + PartialEq
// + SerializableEnum<'de>
where
    Self: Sized,
{
}

pub static STR_ENUM_CLS: GILOnceCell<Py<PyType>> = GILOnceCell::new();

pub fn str_enum_python_class(py: Python<'_>) -> PyResult<Py<PyType>> {
    Ok(STR_ENUM_CLS.import(py, "enum", "StrEnum")?.clone().unbind())
}

macro_rules! pywrap_enum {
    ($mod_name:ident, $enum_name:ident, [$($variant:ident),+ $(,)?]) => {
        pywrap_enum!($mod_name, $enum_name, ShoutySnakeCase, [$($variant),+]);
    };

    ($mod_name:ident, $enum_name:ident, $casing:ident, [$($variant:ident),+ $(,)?]) => {
        pub mod $mod_name {
            use pyo3::prelude::*;
            use pyo3::types::PyType;
            use pyo3::sync::GILOnceCell;
            use std::collections::HashMap;
            use crate::enums::str_enum_python_class;
            use ptolemy::prelude::enum_utils::CasingStyle;

            pub const ENUM_CLS_NAME: &'static str = stringify!($enum_name);

            pub static ENUM_PY_CLS: GILOnceCell<Py<PyType>> = GILOnceCell::new();

            pub fn add_enum_to_module<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
                m.add(ENUM_CLS_NAME, get_enum_py_cls(py)?)?;
                Ok(())
            }

            pub fn get_enum_py_cls(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
                let mut variants: HashMap<String, String> = HashMap::new();
                $(
                    variants.insert(CasingStyle::ShoutySnakeCase.format(stringify!($variant)), ptolemy::models::$enum_name::$variant.into());
                )+

                let py_cls = ENUM_PY_CLS.get_or_init(py, || {
                    str_enum_python_class(py).unwrap()
                        .bind(py)
                        .call1((ENUM_CLS_NAME, variants,)).unwrap()
                        .downcast_into::<PyType>().unwrap()
                        .unbind()
                });

                Ok(py_cls.bind(py))
                }

            #[derive(Clone, Debug, PartialEq)]
            pub struct $enum_name(pub ptolemy::models::$enum_name);
            }

        impl Into<$mod_name::$enum_name> for ptolemy::models::$enum_name {
            fn into(self) -> $mod_name::$enum_name {
                $mod_name::$enum_name(self)
            }
        }

        impl Into<ptolemy::models::$enum_name> for $mod_name::$enum_name {
            fn into(self) -> ptolemy::models::$enum_name {
                self.0
            }
        }

        impl<'py, 'de> crate::enums::PyEnumCompatible<'py, 'de> for $mod_name::$enum_name {}

        impl <'py> FromPyObject<'py> for $mod_name::$enum_name {
                fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
                    let extracted_str = ob.extract::<String>()?;
                    Ok($mod_name::$enum_name(ptolemy::models::$enum_name::try_from(extracted_str).map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))?))
                }
            }

        impl<'py> IntoPyObject<'py> for $mod_name::$enum_name {
            type Target = PyAny;
            type Output = Bound<'py, Self::Target>;
            type Error = PyErr;

            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                let s: String = self.0.into();
                let val = $mod_name::get_enum_py_cls(py)?.call1((s,))?;
                Ok(val)
            }
        }
    }
}

pywrap_enum!(
    api_key_permission,
    ApiKeyPermission,
    [ReadOnly, WriteOnly, ReadWrite]
);
pywrap_enum!(workspace_role, WorkspaceRole, [User, Manager, Admin]);
pywrap_enum!(user_status, UserStatus, [Active, Suspended]);

#[macro_export]
macro_rules! string_enum {
    ($mod_name:ident, $enum_name:ident, [$($variant:ident),+ $(,)?]) => {
        string_enum!($mod_name, $enum_name, SnakeCase, [$($variant),+]);
    };

    ($mod_name:ident, $enum_name:ident, $casing:ident, [$($variant:ident),+ $(,)?]) => {
        pub mod $mod_name {
            use pyo3::prelude::*;
            use pyo3::types::PyType;
            use pyo3::exceptions::PyValueError;
            use pyo3::sync::GILOnceCell;
            use std::collections::HashMap;
            use heck::{ToSnakeCase, ToShoutySnakeCase, ToLowerCamelCase, ToPascalCase};

            fn format_variant(variant: &str) -> String {
                match stringify!($casing) {
                    "ShoutySnakeCase" => variant.to_shouty_snake_case(),
                    "SnakeCase" => variant.to_snake_case(),
                    "CamelCase" => variant.to_lower_camel_case(),
                    "PascalCase" => variant.to_pascal_case(),
                    _ => variant.to_snake_case(),
                }
            }

            const ENUM_CLS_NAME: &'static str = stringify!($enum_name);

            static ENUM_PY_CLS: GILOnceCell<Py<PyType>> = GILOnceCell::new();

            fn get_enum_py_cls(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
                let mut variants = HashMap::new();
                $(
                    variants.insert(stringify!($variant).to_shouty_snake_case(), format_variant(stringify!($variant)));
                )+

                let py_cls = ENUM_PY_CLS.get_or_init(py, || {
                    py
                        .import("enum").unwrap()
                        .getattr("StrEnum").unwrap()
                        .call1((ENUM_CLS_NAME, variants,)).unwrap()
                        .downcast_into::<PyType>().unwrap()
                        .unbind()
                });

                Ok(py_cls.bind(py))
                }

            #[derive(Debug, Clone, PartialEq)]
            pub enum $enum_name {
                $(
                    $variant
                ),+
            }

            impl Into<String> for $enum_name {
                fn into(self) -> String {
                    match self {
                        $(
                            Self::$variant => format_variant(stringify!($variant)),
                        )+
                    }
                }
            }

            impl TryFrom<String> for $enum_name {
                type Error = PyErr;

                fn try_from(value: String) -> Result<Self, Self::Error> {
                    match value.as_str() {
                        $(
                            v if v == format_variant(stringify!($variant)) => Ok(Self::$variant),
                        )+
                        _ => Err(PyValueError::new_err(format!("Invalid enum value: {}", value)))
                    }
                }
            }

            impl <'py> FromPyObject<'py> for $enum_name {
                fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
                    $enum_name::try_from(ob.extract::<String>()?)
                }
            }

            impl<'py> IntoPyObject<'py> for $enum_name {
                type Target = PyAny;
                type Output = Bound<'py, Self::Target>;
                type Error = PyErr;
            
                fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                    let s: String = self.into();
                    let val = get_enum_py_cls(py)?.call1((s,))?;
                    Ok(val)
                }
            }

            pub fn add_enum_to_module<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
                m.add(ENUM_CLS_NAME, get_enum_py_cls(py)?)?;
                Ok(())
            }
        }
    }
}

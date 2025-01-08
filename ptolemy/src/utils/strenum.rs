#[macro_export]
macro_rules! string_enum {
    ($mod_name:ident, $enum_name:ident, [$($variant:ident),+ $(,)?]) => {
        string_enum!($mod_name, $enum_name, SnakeCase, [$($variant),+]);
    };

    ($mod_name:ident, $enum_name:ident, $casing:ident, [$($variant:ident),+ $(,)?]) => {
        pub mod $mod_name {
            use pyo3::prelude::*;
            use once_cell::sync::Lazy;
            use pyo3::types::PyType;
            use pyo3::exceptions::PyValueError;
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

            static ENUM_PY_CLS_NAME: Lazy<String> = Lazy::new(|| stringify!($enum_name).to_pascal_case());

            static VARIANTS: Lazy<HashMap<String, String>> = Lazy::new(|| {
                let mut variants = HashMap::new();
                $(
                    variants.insert(stringify!($variant).to_shouty_snake_case(), format_variant(stringify!($variant)));
                )+

                variants
            });

            static ENUM_PY_CLS: Lazy<Py<PyType>> = Lazy::new(|| {
                let enum_cls: PyResult<Py<PyType>> = Python::with_gil(|py| {
                    let en = py.import("enum")?
                        .getattr("StrEnum")?
                        .call1((stringify!($enum_name).to_pascal_case(), VARIANTS.clone(),))?
                        .downcast::<PyType>()?
                        .clone()
                        .unbind();

                    Ok(en)
                });

                match enum_cls {
                    Ok(ec) => ec,
                    Err(e) => panic!("Failed to create enum class: {}", e),
                }
            });

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
                type Error = std::convert::Infallible;
            
                fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                    let s: String = self.into();
                    let val = ENUM_PY_CLS.bind(py).call1((s,)).unwrap();
                    Ok(val)
                }
            }

            pub fn add_enum_to_module<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
                m.add(ENUM_PY_CLS_NAME.clone(), ENUM_PY_CLS.bind(py))?;
                Ok(())
            }
        }
    }
}

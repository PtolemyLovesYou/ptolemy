#[macro_export]
macro_rules! string_enum {
    // Pattern matching with no attributes
    ($mod_name:ident, $enum_name:ident, [$($variant:ident),+ $(,)?]) => {
        string_enum!($mod_name, $enum_name, [$($variant),+], );
    };

    // Pattern matching with attributes
    ($mod_name:ident, $enum_name: ident, [$($variant:ident),+ $(,)?], $($attr:meta),* $(,)?) => {
        mod $mod_name {
            use pyo3::prelude::*;
            use pyo3::types::{PyString, PyType, PyAny};
            use pyo3::exceptions::PyValueError;
            use pyo3::basic::CompareOp;
            use std::hash::{DefaultHasher, Hash, Hasher};
            use heck::{ToShoutySnakeCase};

            #[pyclass(frozen $(, $attr)*)]
            #[derive(Clone, Debug, PartialEq, PartialOrd)]
            pub enum $enum_name {
                $(
                    $variant
                ),*
            }

            impl TryFrom<$enum_name> for String {
                type Error = PyErr;
                fn try_from(value: $enum_name) -> Result<String, Self::Error> {
                    Ok(value.value())
                }
            }

            impl TryFrom<String> for $enum_name {
                type Error = PyErr;
                fn try_from(value: String) -> Result<$enum_name, Self::Error> {
                    match value.as_str() {
                        $(stringify!($variant) => Ok($enum_name::$variant),)*
                        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, ()>(()))
                    }
                }
            }

            #[derive(FromPyObject)]
            struct Wrapper($enum_name);

            #[pymethods]
            impl $enum_name {
                #[classmethod]
                fn values(_cls: &Bound<'_, PyType>) -> Vec<String> {
                    vec![
                        $(stringify!($variant).to_shouty_snake_case().to_string()),*
                    ]
                }

                #[getter]
                fn value(&self) -> String {
                    match self {
                        $($enum_name::$variant => stringify!($variant).to_shouty_snake_case(),)*
                    }
                }

                fn __str__(&self) -> String {
                    self.value()
                }

                fn __repr__(&self) -> String {
                    format!("{}.{}", stringify!($enum_name), self.value())
                }

                fn __hash__(&self) -> u64 {
                    let val = self.value();
                    let mut hasher = DefaultHasher::new();
                    val.hash(&mut hasher);
                    hasher.finish()
                }

                fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: CompareOp) -> PyResult<bool> {
                    let other = if let Ok(other) = other.downcast::<PyString>() {
                        other.extract::<String>()?
                    } else if let Ok(other) = other.extract::<Wrapper>() {
                        String::try_from(other.0)?
                    } else {
                        return Err(PyValueError::new_err("Invalid value"));
                    };

                    let self_val = String::try_from(self.clone())?;
            
                    match op {
                        CompareOp::Eq => Ok(self_val == other),
                        CompareOp::Ne => Ok(self_val != other),
                        CompareOp::Ge => Ok(self_val >= other),
                        CompareOp::Gt => Ok(self_val > other),
                        CompareOp::Le => Ok(self_val <= other),
                        CompareOp::Lt => Ok(self_val < other),
                    }
                }
            }
        }
    };
}

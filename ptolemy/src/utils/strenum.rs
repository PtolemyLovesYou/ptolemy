use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::sync::GILOnceCell;

pub trait PyEnumCompatible<'py>: 
    IntoPyObject<'py, Target = PyAny, Output = Bound<'py, PyAny>, Error = PyErr> + 
    FromPyObject<'py> +
    Clone +
    PartialEq +
    Into<String> +
    TryFrom<String, Error = PyErr>
where
    Self: Sized
{}

pub static STR_ENUM_CLS: GILOnceCell<Py<PyType>> = GILOnceCell::new();

pub fn str_enum_python_class(py: Python<'_>) -> PyResult<Py<PyType>> {
    Ok(
        STR_ENUM_CLS
            .import(py, "enum", "StrEnum")?
            .clone()
            .unbind()
    )
}


#[macro_export]
macro_rules! string_enum {
    ($mod_name:ident, $enum_name:ident, [$($variant:ident),+ $(,)?]) => {
        string_enum!($mod_name, $enum_name, ShoutySnakeCase, [$($variant),+]);
    };

    ($mod_name:ident, $enum_name:ident, $casing:ident, [$($variant:ident),+ $(,)?]) => {
        pub mod $mod_name {
            use pyo3::prelude::*;
            use pyo3::types::PyType;
            use pyo3::exceptions::PyValueError;
            use pyo3::sync::GILOnceCell;
            use std::collections::HashMap;
            use heck::{ToSnakeCase, ToShoutySnakeCase, ToLowerCamelCase, ToPascalCase};
            use crate::utils::strenum::str_enum_python_class;

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

            pub fn get_enum_py_cls(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
                let mut variants = HashMap::new();
                $(
                    variants.insert(stringify!($variant).to_shouty_snake_case(), format_variant(stringify!($variant)));
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

            #[derive(Debug, Clone, PartialEq)]
            pub enum $enum_name {
                $(
                    $variant
                ),+
            }

            impl<'py> crate::utils::strenum::PyEnumCompatible<'py> for $enum_name {}

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

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use super::*;
    use pyo3::types::PyString;

    static INIT: Once = Once::new();

    string_enum!(my_enum, MyEnum, [Val1, Val2, Val3]);
    string_enum!(my_enum_cased, MyEnumCased, SnakeCase, [ValCasedOne, ValCasedTwo, ValCasedThree]);

    fn init() {
        INIT.call_once(|| {
            pyo3::prepare_freethreaded_python();
        })
    }

    fn get_enum_py_string<'a, T: PyEnumCompatible<'a>>(py: Python<'a>, en: T) -> PyResult<String> {
        Ok(en.into_pyobject(py)?.getattr("value")?.extract()?)
    }

    #[test]
    fn test_strenum_creation_default() {
        init();

        use my_enum::MyEnum;

        Python::with_gil(|py| {
            let val1 = get_enum_py_string(py, MyEnum::Val1).unwrap();
            let val2 = get_enum_py_string(py, MyEnum::Val2).unwrap();
            let val3 = get_enum_py_string(py, MyEnum::Val3).unwrap();

            assert_eq!(val1, "VAL1");
            assert_eq!(val2, "VAL2");
            assert_eq!(val3, "VAL3");
        })
    }

    #[test]
    fn test_strenum_creation_casing() {
        init();

        use my_enum_cased::MyEnumCased;

        Python::with_gil(|py| {
            let val1 = get_enum_py_string(py, MyEnumCased::ValCasedOne).unwrap();
            let val2 = get_enum_py_string(py, MyEnumCased::ValCasedTwo).unwrap();
            let val3 = get_enum_py_string(py, MyEnumCased::ValCasedThree).unwrap();

            assert_eq!(val1, "val_cased_one");
            assert_eq!(val2, "val_cased_two");
            assert_eq!(val3, "val_cased_three");
        })
    }

    #[test]
    fn test_from_string() {
        init();

        Python::with_gil(|py| {
            // test valid values
            let val1 = PyString::new(py, "VAL1");
            let val2 = PyString::new(py, "VAL2");
            let val3 = PyString::new(py, "VAL3");
            
            assert_eq!(val1.extract::<my_enum::MyEnum>().unwrap(), my_enum::MyEnum::Val1);
            assert_eq!(val2.extract::<my_enum::MyEnum>().unwrap(), my_enum::MyEnum::Val2);
            assert_eq!(val3.extract::<my_enum::MyEnum>().unwrap(), my_enum::MyEnum::Val3);

            // test invalid values
            let bad_str = PyString::new(py, "BadValue");
            assert!(bad_str.extract::<my_enum::MyEnum>().is_err());
        })
    }

    #[test]
    fn test_add_enum_to_module() {
        init();

        Python::with_gil(|py| {
            let m = PyModule::new(py, "my_module").unwrap();
            my_enum::add_enum_to_module(py, &m).unwrap();
            my_enum_cased::add_enum_to_module(py, &m).unwrap();
        })
    }

    #[test]
    fn test_py_cls() {
        init();

        Python::with_gil(|py| {
            let pycls = my_enum::get_enum_py_cls(py).expect("Failed to get enum class");

            // test to make sure StrEnum class is iterable
            pycls.extract::<Vec<String>>().expect("Failed to extract Vec<String>");

            // test to make sure that pycls is a subclass of StrEnum
            assert!(pycls.is_subclass(&str_enum_python_class(py).unwrap().bind(py)).expect("Failed to check subclass"));
        })
    }
}

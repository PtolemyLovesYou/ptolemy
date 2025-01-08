use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3::sync::GILOnceCell;
use pyo3::exceptions::PyValueError;
use heck::{ToSnakeCase, ToShoutySnakeCase, ToLowerCamelCase, ToPascalCase};
use crate::models::enums::{ApiKeyPermission, UserStatus, WorkspaceRole};

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

#[derive(Debug)]
pub enum CasingStyle {
    ShoutySnakeCase,
    SnakeCase,
    LowerCamelCase,
    PascalCase
}

impl CasingStyle {
    pub fn format(&self, variant: &str) -> String {
        match self {
            CasingStyle::ShoutySnakeCase => variant.to_shouty_snake_case(),
            CasingStyle::SnakeCase => variant.to_snake_case(),
            CasingStyle::LowerCamelCase => variant.to_lower_camel_case(),
            CasingStyle::PascalCase => variant.to_pascal_case(),
        }
    }
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
            use crate::pybindings::enums::{str_enum_python_class, CasingStyle};

            pub const ENUM_CLS_NAME: &'static str = stringify!($enum_name);

            pub static ENUM_PY_CLS: GILOnceCell<Py<PyType>> = GILOnceCell::new();

            pub fn add_enum_to_module<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
                m.add(ENUM_CLS_NAME, get_enum_py_cls(py)?)?;
                Ok(())
            }

            pub fn get_enum_py_cls(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
                let mut variants = HashMap::new();
                $(
                    variants.insert(CasingStyle::ShoutySnakeCase.format(stringify!($variant)), CasingStyle::$casing.format(stringify!($variant)));
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
            }

        impl<'py> crate::pybindings::enums::PyEnumCompatible<'py> for $enum_name {}

        impl Into<String> for $enum_name {
            fn into(self) -> String {
                match self {
                    $(
                        Self::$variant => CasingStyle::$casing.format(stringify!($variant)),
                    )+
                }
            }
        }

        impl TryFrom<String> for $enum_name {
            type Error = PyErr;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                match value.as_str() {
                    $(
                        v if v == CasingStyle::$casing.format(stringify!($variant)) => Ok(Self::$variant),
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
                let val = $mod_name::get_enum_py_cls(py)?.call1((s,))?;
                Ok(val)
            }
    }
}
}

pywrap_enum!(api_key_permission, ApiKeyPermission, [ReadOnly, WriteOnly, ReadWrite]);
pywrap_enum!(workspace_role, WorkspaceRole, [User, Manager, Admin]);
pywrap_enum!(user_status, UserStatus, [Active, Suspended]);

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use super::*;
    use pyo3::types::PyString;

    static INIT: Once = Once::new();

    #[derive(Clone, Debug, PartialEq)]
    pub enum MyEnum {
        Val1,
        Val2,
        Val3,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum MyEnumCased {
        ValCasedOne,
        ValCasedTwo,
        ValCasedThree,
    }

    pywrap_enum!(my_enum, MyEnum, [Val1, Val2, Val3]);
    pywrap_enum!(my_enum_cased, MyEnumCased, SnakeCase, [ValCasedOne, ValCasedTwo, ValCasedThree]);

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
            
            assert_eq!(val1.extract::<MyEnum>().unwrap(), MyEnum::Val1);
            assert_eq!(val2.extract::<MyEnum>().unwrap(), MyEnum::Val2);
            assert_eq!(val3.extract::<MyEnum>().unwrap(), MyEnum::Val3);

            // test invalid values
            let bad_str = PyString::new(py, "BadValue");
            assert!(bad_str.extract::<MyEnum>().is_err());
        })
    }

    #[test]
    fn test_into_pyany() {
        init();

        Python::with_gil(|py| {
            let val1_to = MyEnum::Val1.into_pyobject(py).expect("Failed to convert to PyAny");
            let val1_from = my_enum::get_enum_py_cls(py).unwrap().getattr("VAL1").expect("Failed to get attribute");

            assert!(val1_to.is_instance(&val1_from.get_type()).expect("Couldn't compare :("));
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
            let variants_list: PyResult<Vec<String>> = pycls.try_iter()
                .expect("Couldn't turn into Iterator!")
                .map(|item| item.unwrap().extract::<String>())
                .collect();

            assert!(variants_list.is_ok());

            let str_enum_cls = py.import("enum").unwrap().getattr("StrEnum").unwrap();

            // test to make sure that pycls is a subclass of StrEnum
            assert!(pycls.is_subclass(&str_enum_cls).expect("Failed to check subclass"));
        })
    }
}

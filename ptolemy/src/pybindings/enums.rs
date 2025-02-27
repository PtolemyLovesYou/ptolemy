use crate::models::{ApiKeyPermission, UserStatus, WorkspaceRole};
use crate::prelude::enum_utils::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::sync::GILOnceCell;
use pyo3::types::PyType;

pub trait PyEnumCompatible<'py, 'de>:
    IntoPyObject<'py, Target = PyAny, Output = Bound<'py, PyAny>, Error = PyErr>
    + FromPyObject<'py>
    + Clone
    + PartialEq
    + SerializableEnum<'de>
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
            use crate::pybindings::enums::str_enum_python_class;
            use crate::prelude::enum_utils::CasingStyle;
            use super::$enum_name;

            pub const ENUM_CLS_NAME: &'static str = stringify!($enum_name);

            pub static ENUM_PY_CLS: GILOnceCell<Py<PyType>> = GILOnceCell::new();

            pub fn add_enum_to_module<'a>(py: Python<'a>, m: &Bound<'a, PyModule>) -> PyResult<()> {
                m.add(ENUM_CLS_NAME, get_enum_py_cls(py)?)?;
                Ok(())
            }

            pub fn get_enum_py_cls(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
                let mut variants: HashMap<String, String> = HashMap::new();
                $(
                    variants.insert(CasingStyle::ShoutySnakeCase.format(stringify!($variant)), $enum_name::$variant.into());
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

        impl<'py, 'de> crate::pybindings::enums::PyEnumCompatible<'py, 'de> for $enum_name {}

        impl <'py> FromPyObject<'py> for $enum_name {
            fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
                let extracted_str = ob.extract::<String>()?;
                $enum_name::try_from(extracted_str).map_err(|e| PyValueError::new_err(format!("{:?}", e)))
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

pywrap_enum!(
    api_key_permission,
    ApiKeyPermission,
    [ReadOnly, WriteOnly, ReadWrite]
);
pywrap_enum!(workspace_role, WorkspaceRole, [User, Manager, Admin]);
pywrap_enum!(user_status, UserStatus, [Active, Suspended]);

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::types::PyString;
    use std::sync::Once;

    static INIT: Once = Once::new();

    #[derive(Clone, Debug, PartialEq)]
    pub enum MyEnum {
        Foo,
        Bar,
        Baz,
    }

    serialize_enum!(MyEnum, ShoutySnakeCase, [Foo, Bar, Baz]);

    #[derive(Clone, Debug, PartialEq)]
    pub enum MyEnumCased {
        ValCasedOne,
        ValCasedTwo,
        ValCasedThree,
    }

    serialize_enum!(
        MyEnumCased,
        SnakeCase,
        [ValCasedOne, ValCasedTwo, ValCasedThree]
    );

    pywrap_enum!(my_enum, MyEnum, [Foo, Bar, Baz]);
    pywrap_enum!(
        my_enum_cased,
        MyEnumCased,
        SnakeCase,
        [ValCasedOne, ValCasedTwo, ValCasedThree]
    );

    fn init() {
        INIT.call_once(|| {
            pyo3::prepare_freethreaded_python();
        })
    }

    fn get_enum_py_string<'py, 'a: 'py, T: PyEnumCompatible<'py, 'a>>(
        py: Python<'a>,
        en: T,
    ) -> PyResult<String> {
        Ok(en.into_pyobject(py)?.getattr("value")?.extract()?)
    }

    #[test]
    fn test_strenum_creation_default() {
        init();

        Python::with_gil(|py| {
            let val1 = get_enum_py_string(py, MyEnum::Foo).unwrap();
            let val2 = get_enum_py_string(py, MyEnum::Bar).unwrap();
            let val3 = get_enum_py_string(py, MyEnum::Baz).unwrap();

            assert_eq!(val1, "FOO");
            assert_eq!(val2, "BAR");
            assert_eq!(val3, "BAZ");
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
            let val1 = PyString::new(py, "FOO");
            let val2 = PyString::new(py, "BAR");
            let val3 = PyString::new(py, "BAZ");

            assert_eq!(val1.extract::<MyEnum>().unwrap(), MyEnum::Foo);
            assert_eq!(val2.extract::<MyEnum>().unwrap(), MyEnum::Bar);
            assert_eq!(val3.extract::<MyEnum>().unwrap(), MyEnum::Baz);

            // test invalid values
            let bad_str = PyString::new(py, "BadValue");
            assert!(bad_str.extract::<MyEnum>().is_err());
        })
    }

    #[test]
    fn test_into_pyany() {
        init();

        Python::with_gil(|py| {
            let val1_to = MyEnum::Foo
                .into_pyobject(py)
                .expect("Failed to convert to PyAny");
            let val1_from = my_enum::get_enum_py_cls(py)
                .unwrap()
                .getattr("FOO")
                .expect("Failed to get attribute");

            assert!(val1_to
                .is_instance(&val1_from.get_type())
                .expect("Couldn't compare :("));
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
            let variants_list: PyResult<Vec<String>> = pycls
                .try_iter()
                .expect("Couldn't turn into Iterator!")
                .map(|item| item.unwrap().extract::<String>())
                .collect();

            assert!(variants_list.is_ok());

            let str_enum_cls = py.import("enum").unwrap().getattr("StrEnum").unwrap();

            // test to make sure that pycls is a subclass of StrEnum
            assert!(pycls
                .is_subclass(&str_enum_cls)
                .expect("Failed to check subclass"));
        })
    }
}

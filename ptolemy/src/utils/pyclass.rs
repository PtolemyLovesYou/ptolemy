#[macro_export]
macro_rules! pymodel {
    ($struct:ty, $name:ident, [$($getter:ident),+ $(,)?]) => {
        #[pyclass]
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

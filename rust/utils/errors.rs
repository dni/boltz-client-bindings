use pyo3::exceptions::PyValueError;
use pyo3::{PyErr, PyTypeInfo};

pub fn to_python_error<E: PyTypeInfo, T: std::fmt::Debug>(msg: &str, err: T) -> PyErr {
    PyErr::new::<E, _>(format!("{}: {:?}", msg, err))
}

pub fn handle_rust_error<T, E: std::fmt::Debug>(msg: &str, res: Result<T, E>) -> Result<T, PyErr> {
    match res {
        Ok(r) => Ok(r),
        Err(err) => Err(to_python_error::<PyValueError, _>(msg, err)),
    }
}

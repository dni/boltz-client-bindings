use pyo3::prelude::*;
use boltz_client::swaps::boltz::{SwapStatusRequest};

#[pyclass]
struct SwapStatusRequest {
    #[pyo3(get, set)]
    pub id: String,
}

#[pyfunction]
fn lol() -> PyResult<String> {
    Ok("lol".to_string())
}

#[pymodule]
fn _lib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(lol, m)?)?;
    m.add_class::<SwapStatusRequest>()?;
    Ok(())
}

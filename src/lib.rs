use pyo3::prelude::PyModule;
use pyo3::{pymodule, wrap_pyfunction, Bound, PyResult};

use swaps::bitcoin;

mod client;
mod swaps;
mod types;
mod utils;

#[pymodule]
fn boltz_client_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<bitcoin::BtcSwapScript>()?;
    m.add_class::<types::submarine::CreateSubmarineResponse>()?;
    m.add_class::<types::submarine::SwapTree>()?;
    m.add_class::<types::submarine::Leaf>()?;
    m.add_class::<client::boltz::Client>()?;
    m.add_class::<types::client::SwapResponse>()?;
    m.add_class::<types::client::HeightResponse>()?;
    m.add_class::<types::client::Fees>()?;
    m.add_class::<types::client::Limits>()?;
    m.add_class::<types::client::SwapParams>()?;
    m.add_function(wrap_pyfunction!(utils::keys::new_keys, m)?)?;
    m.add_function(wrap_pyfunction!(utils::address::validate_address, m)?)?;
    Ok(())
}

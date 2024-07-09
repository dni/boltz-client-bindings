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
    m.add_class::<types::submarine::CreateReverseResponse>()?;
    m.add_class::<types::submarine::SwapTree>()?;
    m.add_class::<types::submarine::Leaf>()?;
    m.add_class::<client::boltz::Client>()?;
    m.add_class::<types::client::GetSubmarinePairsResponse>()?;
    m.add_class::<types::client::HeightResponse>()?;
    m.add_class::<types::client::SubmarineFees>()?;
    m.add_class::<types::client::PairLimits>()?;
    m.add_class::<types::client::SubmarinePair>()?;
    m.add_function(wrap_pyfunction!(utils::keys::new_keys, m)?)?;
    m.add_function(wrap_pyfunction!(utils::address::validate_address, m)?)?;
    Ok(())
}

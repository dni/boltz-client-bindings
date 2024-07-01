use std::collections::HashMap;

use pyo3::{Bound, pyclass, pymethods, PyResult, Python};
use pyo3::prelude::PyDictMethods;
use pyo3::types::PyDict;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Limits {
    #[pyo3(get)]
    pub maximal: u64,
    #[pyo3(get)]
    pub minimal: u64,
    #[pyo3(get)]
    pub maximal_zero_conf: u64,
}

#[pymethods]
impl Limits {
    #[new]
    pub fn new(maximal: u64, minimal: u64, maximal_zero_conf: u64) -> Self {
        Limits {
            maximal,
            minimal,
            maximal_zero_conf,
        }
    }
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("maximal", self.maximal)?;
        dict.set_item("minimal", self.minimal)?;
        dict.set_item("maximal_zero_conf", self.maximal_zero_conf)?;
        Ok(dict)
    }
}

impl From<boltz_client::swaps::boltzv2::Limits> for Limits {
    fn from(value: boltz_client::swaps::boltzv2::Limits) -> Self {
        Limits {
            maximal: value.maximal,
            minimal: value.minimal,
            maximal_zero_conf: value.maximal_zero_conf,
        }
    }
}

impl From<Limits> for boltz_client::swaps::boltzv2::Limits {
    fn from(value: Limits) -> Self {
        boltz_client::swaps::boltzv2::Limits {
            maximal: value.maximal,
            minimal: value.minimal,
            maximal_zero_conf: value.maximal_zero_conf,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Fees {
    #[pyo3(get)]
    pub percentage: f64,
    #[pyo3(get)]
    pub miner_fees: u64,
}

#[pymethods]
impl Fees {
    #[new]
    pub fn new(percentage: f64, miner_fees: u64) -> Self {
        Fees {
            percentage,
            miner_fees,
        }
    }
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("percentage", self.percentage)?;
        dict.set_item("miner_fees", self.miner_fees)?;
        Ok(dict)
    }
}

impl From<boltz_client::swaps::boltzv2::Fees> for Fees {
    fn from(value: boltz_client::swaps::boltzv2::Fees) -> Self {
        Fees {
            percentage: value.percentage,
            miner_fees: value.miner_fees,
        }
    }
}

impl From<Fees> for boltz_client::swaps::boltzv2::Fees {
    fn from(value: Fees) -> Self {
        boltz_client::swaps::boltzv2::Fees {
            percentage: value.percentage,
            miner_fees: value.miner_fees,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct SwapParams {
    #[pyo3(get)]
    pub hash: String,
    #[pyo3(get)]
    pub rate: f64,
    #[pyo3(get)]
    pub limits: Limits,
    #[pyo3(get)]
    pub fees: Fees,
}

#[pymethods]
impl SwapParams {
    #[new]
    pub fn new(hash: String, rate: f64, limits: Limits, fees: Fees) -> Self {
        SwapParams {
            hash,
            rate,
            limits,
            fees,
        }
    }
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("hash", self.hash.clone())?;
        dict.set_item("rate", self.rate)?;
        dict.set_item("limits", self.limits.to_dict(py)?)?;
        dict.set_item("fees", self.fees.to_dict(py)?)?;
        Ok(dict)
    }
}

impl From<boltz_client::swaps::boltzv2::SwapParams> for SwapParams {
    fn from(value: boltz_client::swaps::boltzv2::SwapParams) -> Self {
        SwapParams {
            hash: value.hash,
            rate: value.rate,
            limits: value.limits.into(),
            fees: value.fees.into(),
        }
    }
}

impl From<SwapParams> for boltz_client::swaps::boltzv2::SwapParams {
    fn from(value: SwapParams) -> Self {
        boltz_client::swaps::boltzv2::SwapParams {
            hash: value.hash,
            rate: value.rate,
            limits: value.limits.into(),
            fees: value.fees.into(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct SwapResponse {
    #[pyo3(get)]
    pub btc: HashMap<String, SwapParams>,
    #[pyo3(get)]
    pub lbtc: HashMap<String, SwapParams>,
}

#[pymethods]
impl SwapResponse {
    #[new]
    pub fn new(btc: HashMap<String, SwapParams>, lbtc: HashMap<String, SwapParams>) -> Self {
        SwapResponse { btc, lbtc }
    }
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        let btc = PyDict::new_bound(py);
        for (key, value) in &self.btc {
            btc.set_item(key, value.to_dict(py)?)?;
        }
        let lbtc = PyDict::new_bound(py);
        for (key, value) in &self.lbtc {
            lbtc.set_item(key, value.to_dict(py)?)?;
        }
        dict.set_item("btc", btc)?;
        dict.set_item("lbtc", lbtc)?;
        Ok(dict)
    }
}

impl From<boltz_client::swaps::boltzv2::SwapResponse> for SwapResponse {
    fn from(value: boltz_client::swaps::boltzv2::SwapResponse) -> Self {
        let mut btc = HashMap::new();
        for (key, value) in value.btc {
            btc.insert(key, value.into());
        }
        let mut lbtc = HashMap::new();
        for (key, value) in value.lbtc {
            lbtc.insert(key, value.into());
        }
        SwapResponse { btc, lbtc }
    }
}

impl From<SwapResponse> for boltz_client::swaps::boltzv2::SwapResponse {
    fn from(value: SwapResponse) -> Self {
        let mut btc = HashMap::new();
        for (key, value) in value.btc {
            btc.insert(key, value.into());
        }
        let mut lbtc = HashMap::new();
        for (key, value) in value.lbtc {
            lbtc.insert(key, value.into());
        }
        boltz_client::swaps::boltzv2::SwapResponse { btc, lbtc }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct HeightResponse {
    #[pyo3(get)]
    pub btc: u32,
    #[pyo3(get)]
    pub lbtc: u32,
}

#[pymethods]
impl HeightResponse {
    #[new]
    pub fn new(btc: u32, lbtc: u32) -> Self {
        HeightResponse { btc, lbtc }
    }
}

impl From<HeightResponse> for boltz_client::swaps::boltzv2::HeightResponse {
    fn from(value: HeightResponse) -> Self {
        boltz_client::swaps::boltzv2::HeightResponse {
            btc: value.btc,
            lbtc: value.lbtc,
        }
    }
}

impl From<boltz_client::swaps::boltzv2::HeightResponse> for HeightResponse {
    fn from(value: boltz_client::swaps::boltzv2::HeightResponse) -> Self {
        HeightResponse {
            btc: value.btc,
            lbtc: value.lbtc,
        }
    }
}

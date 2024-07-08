use std::collections::HashMap;

use pyo3::{Bound, pyclass, pymethods, PyResult, Python};
use pyo3::prelude::PyDictMethods;
use pyo3::types::PyDict;


#[pyclass]
#[derive(Debug, Clone)]
pub struct ReverseLimits {
    #[pyo3(get)]
    pub maximal: u64,
    #[pyo3(get)]
    pub minimal: u64,
}


#[pymethods]
impl ReverseLimits {
    #[new]
    pub fn new(maximal: u64, minimal: u64) -> Self {
        ReverseLimits {
            maximal,
            minimal,
        }
    }
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("maximal", self.maximal)?;
        dict.set_item("minimal", self.minimal)?;
        Ok(dict)
    }
}

impl From<boltz_client::swaps::boltz::ReverseLimits> for ReverseLimits {
    fn from(value: boltz_client::swaps::boltz::ReverseLimits) -> Self {
        ReverseLimits {
            maximal: value.maximal,
            minimal: value.minimal,
        }
    }
}

impl From<ReverseLimits> for boltz_client::swaps::boltz::ReverseLimits {
    fn from(value: ReverseLimits) -> Self {
        boltz_client::swaps::boltz::ReverseLimits {
            maximal: value.maximal,
            minimal: value.minimal,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ReverseFees {
    #[pyo3(get)]
    pub percentage: f64,
    #[pyo3(get)]
    pub miner_fees: PairMinerFees,
}

#[pymethods]
impl ReverseFees {
    #[new]
    pub fn new(percentage: f64, miner_fees: PairMinerFees) -> Self {
        ReverseFees {
            percentage,
            miner_fees,
        }
    }
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("percentage", self.percentage)?;
        dict.set_item("miner_fees", self.miner_fees.to_dict(py)?)?;
        Ok(dict)
    }
}

impl From<boltz_client::swaps::boltz::ReverseFees> for ReverseFees {
    fn from(value: boltz_client::swaps::boltz::ReverseFees) -> Self {
        ReverseFees {
            percentage: value.percentage,
            miner_fees: value.miner_fees.into(),
        }
    }
}

impl From<ReverseFees> for boltz_client::swaps::boltz::ReverseFees {
    fn from(value: ReverseFees) -> Self {
        boltz_client::swaps::boltz::ReverseFees {
            percentage: value.percentage,
            miner_fees: value.miner_fees.into(),
        }
    }
}


#[pyclass]
#[derive(Debug, Clone)]
pub struct ReversePair {
    #[pyo3(get)]
    pub hash: String,
    #[pyo3(get)]
    pub rate: f64,
    #[pyo3(get)]
    pub limits: ReverseLimits,
    #[pyo3(get)]
    pub fees: ReverseFees,
}

#[pymethods]
impl ReversePair {
    #[new]
    pub fn new(hash: String, rate: f64, limits: ReverseLimits, fees: ReverseFees) -> Self {
        ReversePair {
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

impl From<boltz_client::swaps::boltz::ReversePair> for ReversePair {
    fn from(value: boltz_client::swaps::boltz::ReversePair) -> Self {
        ReversePair {
            hash: value.hash,
            rate: value.rate,
            limits: value.limits.into(),
            fees: value.fees.into(),
        }
    }
}

impl From<ReversePair> for boltz_client::swaps::boltz::ReversePair {
    fn from(value: ReversePair) -> Self {
        boltz_client::swaps::boltz::ReversePair {
            hash: value.hash,
            rate: value.rate,
            limits: value.limits.into(),
            fees: value.fees.into(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct GetReversePairsResponse {
    #[pyo3(get)]
    pub btc: HashMap<String, ReversePair>,
}

#[pymethods]
impl GetReversePairsResponse {
    #[new]
    pub fn new(btc: HashMap<String, ReversePair>) -> Self {
        GetReversePairsResponse { btc }
    }
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        let btc = PyDict::new_bound(py);
        for (key, value) in &self.btc {
            btc.set_item(key, value.to_dict(py)?)?;
        }
        dict.set_item("btc", btc)?;
        Ok(dict)
    }
}

impl From<boltz_client::swaps::boltz::GetReversePairsResponse> for GetReversePairsResponse {
    fn from(value: boltz_client::swaps::boltz::GetReversePairsResponse) -> Self {
        let mut btc = HashMap::new();
        for (key, value) in value.btc {
            btc.insert(key, value.into());
        }
        GetReversePairsResponse { btc }
    }
}

impl From<GetReversePairsResponse> for boltz_client::swaps::boltz::GetReversePairsResponse {
    fn from(value: GetReversePairsResponse) -> Self {
        let mut btc = HashMap::new();
        for (key, value) in value.btc {
            btc.insert(key, value.into());
        }
        boltz_client::swaps::boltz::GetReversePairsResponse { btc }
    }
}


#[pyclass]
#[derive(Debug, Clone)]
pub struct PairMinerFees {
    #[pyo3(get)]
    pub lockup: u64,
    #[pyo3(get)]
    pub claim: u64,
}

#[pymethods]
impl PairMinerFees {
    #[new]
    pub fn new(lockup: u64, claim: u64) -> Self {
        PairMinerFees {
            lockup,
            claim,
        }
    }
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("lockup", self.lockup)?;
        dict.set_item("claim", self.claim)?;
        Ok(dict)
    }
}

impl From<boltz_client::swaps::boltz::PairMinerFees> for PairMinerFees {
    fn from(value: boltz_client::swaps::boltz::PairMinerFees) -> Self {
        PairMinerFees {
            lockup: value.lockup,
            claim: value.claim,
        }
    }
}

impl From<PairMinerFees> for boltz_client::swaps::boltz::PairMinerFees {
    fn from(value: PairMinerFees) -> Self {
        boltz_client::swaps::boltz::PairMinerFees {
            lockup: value.lockup,
            claim: value.claim,
        }
    }
}


#[pyclass]
#[derive(Debug, Clone)]
pub struct PairLimits {
    #[pyo3(get)]
    pub maximal: u64,
    #[pyo3(get)]
    pub minimal: u64,
    #[pyo3(get)]
    pub maximal_zero_conf: u64,
}

#[pymethods]
impl PairLimits {
    #[new]
    pub fn new(maximal: u64, minimal: u64, maximal_zero_conf: u64) -> Self {
        PairLimits {
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

impl From<boltz_client::swaps::boltz::PairLimits> for PairLimits {
    fn from(value: boltz_client::swaps::boltz::PairLimits) -> Self {
        PairLimits {
            maximal: value.maximal,
            minimal: value.minimal,
            maximal_zero_conf: value.maximal_zero_conf,
        }
    }
}

impl From<PairLimits> for boltz_client::swaps::boltz::PairLimits {
    fn from(value: PairLimits) -> Self {
        boltz_client::swaps::boltz::PairLimits {
            maximal: value.maximal,
            minimal: value.minimal,
            maximal_zero_conf: value.maximal_zero_conf,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct SubmarineFees {
    #[pyo3(get)]
    pub percentage: f64,
    #[pyo3(get)]
    pub miner_fees: u64,
}

#[pymethods]
impl SubmarineFees {
    #[new]
    pub fn new(percentage: f64, miner_fees: u64) -> Self {
        SubmarineFees {
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

impl From<boltz_client::swaps::boltz::SubmarineFees> for SubmarineFees {
    fn from(value: boltz_client::swaps::boltz::SubmarineFees) -> Self {
        SubmarineFees {
            percentage: value.percentage,
            miner_fees: value.miner_fees,
        }
    }
}

impl From<SubmarineFees> for boltz_client::swaps::boltz::SubmarineFees {
    fn from(value: SubmarineFees) -> Self {
        boltz_client::swaps::boltz::SubmarineFees {
            percentage: value.percentage,
            miner_fees: value.miner_fees,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct SubmarinePair {
    #[pyo3(get)]
    pub hash: String,
    #[pyo3(get)]
    pub rate: f64,
    #[pyo3(get)]
    pub limits: PairLimits,
    #[pyo3(get)]
    pub fees: SubmarineFees,
}

#[pymethods]
impl SubmarinePair {
    #[new]
    pub fn new(hash: String, rate: f64, limits: PairLimits, fees: SubmarineFees) -> Self {
        SubmarinePair {
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

impl From<boltz_client::swaps::boltz::SubmarinePair> for SubmarinePair {
    fn from(value: boltz_client::swaps::boltz::SubmarinePair) -> Self {
        SubmarinePair {
            hash: value.hash,
            rate: value.rate,
            limits: value.limits.into(),
            fees: value.fees.into(),
        }
    }
}

impl From<SubmarinePair> for boltz_client::swaps::boltz::SubmarinePair {
    fn from(value: SubmarinePair) -> Self {
        boltz_client::swaps::boltz::SubmarinePair {
            hash: value.hash,
            rate: value.rate,
            limits: value.limits.into(),
            fees: value.fees.into(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct GetSubmarinePairsResponse {
    #[pyo3(get)]
    pub btc: HashMap<String, SubmarinePair>,
    #[pyo3(get)]
    pub lbtc: HashMap<String, SubmarinePair>,
}

#[pymethods]
impl GetSubmarinePairsResponse {
    #[new]
    pub fn new(btc: HashMap<String, SubmarinePair>, lbtc: HashMap<String, SubmarinePair>) -> Self {
        GetSubmarinePairsResponse { btc, lbtc }
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

impl From<boltz_client::swaps::boltz::GetSubmarinePairsResponse> for GetSubmarinePairsResponse {
    fn from(value: boltz_client::swaps::boltz::GetSubmarinePairsResponse) -> Self {
        let mut btc = HashMap::new();
        for (key, value) in value.btc {
            btc.insert(key, value.into());
        }
        let mut lbtc = HashMap::new();
        for (key, value) in value.lbtc {
            lbtc.insert(key, value.into());
        }
        GetSubmarinePairsResponse { btc, lbtc }
    }
}

impl From<GetSubmarinePairsResponse> for boltz_client::swaps::boltz::GetSubmarinePairsResponse {
    fn from(value: GetSubmarinePairsResponse) -> Self {
        let mut btc = HashMap::new();
        for (key, value) in value.btc {
            btc.insert(key, value.into());
        }
        let mut lbtc = HashMap::new();
        for (key, value) in value.lbtc {
            lbtc.insert(key, value.into());
        }
        boltz_client::swaps::boltz::GetSubmarinePairsResponse { btc, lbtc }
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

impl From<HeightResponse> for boltz_client::swaps::boltz::HeightResponse {
    fn from(value: HeightResponse) -> Self {
        boltz_client::swaps::boltz::HeightResponse {
            btc: value.btc,
            lbtc: value.lbtc,
        }
    }
}

impl From<boltz_client::swaps::boltz::HeightResponse> for HeightResponse {
    fn from(value: boltz_client::swaps::boltz::HeightResponse) -> Self {
        HeightResponse {
            btc: value.btc,
            lbtc: value.lbtc,
        }
    }
}

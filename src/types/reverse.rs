use pyo3::{pyclass, pymethods, PyErr, Python, Bound, PyResult};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyDictMethods;
use pyo3::types::PyDict;

use crate::utils::keys::{parse_preimage, parse_public_key, parse_chain};
use crate::types::submarine::SwapTree;
use crate::utils::errors::to_python_error;

#[pyclass]
#[derive(Debug, Clone)]
pub struct CreateReverseResponse {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub invoice: String,
    #[pyo3(get)]
    pub swap_tree: SwapTree,
    #[pyo3(get)]
    pub lockup_address: String,
    #[pyo3(get)]
    pub refund_public_key: Vec<u8>,
    #[pyo3(get)]
    pub timeout_block_height: u32,
    #[pyo3(get)]
    pub onchain_amount: u32,
    #[pyo3(get)]
    pub blinding_key: Option<String>,
}

#[pymethods]
impl CreateReverseResponse {

    #[new]
    pub fn new(
        id: String,
        invoice: String,
        swap_tree: SwapTree,
        lockup_address: String,
        refund_public_key: Vec<u8>,
        timeout_block_height: u32,
        onchain_amount: u32,
        blinding_key: Option<String>,
    ) -> Self {
        CreateReverseResponse {
            id,
            invoice,
            swap_tree,
            lockup_address,
            refund_public_key,
            timeout_block_height,
            onchain_amount,
            blinding_key,
        }
    }

    pub fn validate(&self, preimage: Vec<u8>, our_pubkey: Vec<u8>, chain: String) -> PyResult<()> {
        let response = boltz_client::swaps::boltz::CreateReverseResponse::try_from(self.clone())?;
        let _preimage = parse_preimage(preimage)?;
        let _pubkey = parse_public_key(our_pubkey)?;
        let _chain = parse_chain(chain)?;
        match response.validate(&_preimage, &_pubkey, _chain) {
            Ok(_) => Ok(()),
            Err(err) => {
                return Err(to_python_error::<PyValueError, _>(
                    "could not validate response",
                    err,
                ));
            }
        }
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("id", self.id.clone())?;
        dict.set_item("invoice", self.invoice.clone())?;
        dict.set_item("swap_tree", self.swap_tree.to_dict(py)?)?;
        dict.set_item("lockup_address", self.lockup_address.clone())?;
        dict.set_item("refund_public_key", self.refund_public_key.clone())?;
        dict.set_item("timeout_block_height", self.timeout_block_height)?;
        dict.set_item("onchain_amount", self.onchain_amount)?;
        dict.set_item("blinding_key", self.blinding_key.clone())?;
        Ok(dict.into())
    }
}

impl TryFrom<CreateReverseResponse> for boltz_client::swaps::boltz::CreateReverseResponse {
    type Error = PyErr;

    fn try_from(value: CreateReverseResponse) -> Result<Self, Self::Error> {
        Ok(boltz_client::swaps::boltz::CreateReverseResponse {
            id: value.id,
            invoice: value.invoice,
            swap_tree: value.swap_tree.into(),
            lockup_address: value.lockup_address,
            refund_public_key: parse_public_key(value.refund_public_key)?,
            timeout_block_height: value.timeout_block_height,
            onchain_amount: value.onchain_amount,
            blinding_key: value.blinding_key,
        })
    }
}

impl From<boltz_client::swaps::boltz::CreateReverseResponse> for CreateReverseResponse {
    fn from(value: boltz_client::swaps::boltz::CreateReverseResponse) -> Self {
        CreateReverseResponse {
            id: value.id,
            invoice: value.invoice,
            swap_tree: value.swap_tree.into(),
            lockup_address: value.lockup_address,
            refund_public_key: value.refund_public_key.to_bytes(),
            timeout_block_height: value.timeout_block_height,
            onchain_amount: value.onchain_amount,
            blinding_key: value.blinding_key,
        }
    }
}

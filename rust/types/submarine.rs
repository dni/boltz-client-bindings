use pyo3::{pyclass, pymethods, PyErr};

use crate::utils::keys::parse_public_key;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Leaf {
    #[pyo3(get)]
    pub output: String,
    #[pyo3(get)]
    pub version: u8,
}

#[pymethods]
impl Leaf {
    #[new]
    pub fn new(output: String, version: u8) -> Self {
        Leaf { output, version }
    }
}

impl From<Leaf> for boltz_client::swaps::boltzv2::Leaf {
    fn from(value: Leaf) -> Self {
        boltz_client::swaps::boltzv2::Leaf {
            output: value.output,
            version: value.version,
        }
    }
}

impl From<boltz_client::swaps::boltzv2::Leaf> for Leaf {
    fn from(value: boltz_client::swaps::boltzv2::Leaf) -> Self {
        Leaf {
            output: value.output,
            version: value.version,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct SwapTree {
    #[pyo3(get)]
    pub claim_leaf: Leaf,
    #[pyo3(get)]
    pub refund_leaf: Leaf,
}

#[pymethods]
impl SwapTree {
    #[new]
    pub fn new(claim_leaf: Leaf, refund_leaf: Leaf) -> Self {
        SwapTree {
            claim_leaf,
            refund_leaf,
        }
    }
}

impl From<SwapTree> for boltz_client::swaps::boltzv2::SwapTree {
    fn from(value: SwapTree) -> Self {
        boltz_client::swaps::boltzv2::SwapTree {
            claim_leaf: value.claim_leaf.into(),
            refund_leaf: value.refund_leaf.into(),
        }
    }
}

impl From<boltz_client::swaps::boltzv2::SwapTree> for SwapTree {
    fn from(value: boltz_client::swaps::boltzv2::SwapTree) -> Self {
        SwapTree {
            claim_leaf: value.claim_leaf.into(),
            refund_leaf: value.refund_leaf.into(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct CreateSubmarineResponse {
    #[pyo3(get)]
    pub accept_zero_conf: bool,
    #[pyo3(get)]
    pub address: String,
    #[pyo3(get)]
    pub bip21: String,
    #[pyo3(get)]
    pub claim_public_key: Vec<u8>,
    #[pyo3(get)]
    pub expected_amount: u64,
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub blinding_key: Option<String>,
    #[pyo3(get)]
    pub swap_tree: SwapTree,
}

#[pymethods]
impl CreateSubmarineResponse {
    #[new]
    pub fn new(
        accept_zero_conf: bool,
        address: String,
        bip21: String,
        claim_public_key: Vec<u8>,
        expected_amount: u64,
        id: String,
        swap_tree: SwapTree,
        blinding_key: Option<String>,
    ) -> Self {
        CreateSubmarineResponse {
            accept_zero_conf,
            address,
            bip21,
            claim_public_key,
            expected_amount,
            id,
            blinding_key,
            swap_tree,
        }
    }
}

impl TryFrom<CreateSubmarineResponse> for boltz_client::swaps::boltzv2::CreateSubmarineResponse {
    type Error = PyErr;

    fn try_from(value: CreateSubmarineResponse) -> Result<Self, Self::Error> {
        Ok(boltz_client::swaps::boltzv2::CreateSubmarineResponse {
            id: value.id,
            bip21: value.bip21,
            address: value.address,
            expected_amount: value.expected_amount,
            accept_zero_conf: value.accept_zero_conf,
            claim_public_key: parse_public_key(value.claim_public_key)?,
            blinding_key: value.blinding_key,
            swap_tree: value.swap_tree.into(),
        })
    }
}

impl From<boltz_client::swaps::boltzv2::CreateSubmarineResponse> for CreateSubmarineResponse {
    fn from(value: boltz_client::swaps::boltzv2::CreateSubmarineResponse) -> Self {
        CreateSubmarineResponse {
            id: value.id,
            bip21: value.bip21,
            address: value.address,
            expected_amount: value.expected_amount,
            accept_zero_conf: value.accept_zero_conf,
            claim_public_key: value.claim_public_key.to_bytes(),
            blinding_key: value.blinding_key,
            swap_tree: value.swap_tree.into(),
        }
    }
}

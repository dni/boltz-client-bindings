use boltz_client::boltz::SwapType;
use pyo3::{pyclass, pymethods, PyResult};

use crate::types::submarine::CreateSubmarineResponse;
use crate::utils::errors::handle_rust_error;
use crate::utils::keys::parse_public_key;

#[pyclass]
pub struct BtcSwapScript {
    script: boltz_client::BtcSwapScript,
}

#[pymethods]
impl BtcSwapScript {
    #[staticmethod]
    fn from_submarine_response(
        created_response: CreateSubmarineResponse,
        our_pubkey: Vec<u8>,
    ) -> PyResult<Self> {
        Ok(BtcSwapScript {
            script: handle_rust_error(
                "could not parse response",
                boltz_client::BtcSwapScript::submarine_from_swap_resp(
                    &created_response.try_into()?,
                    parse_public_key(our_pubkey)?,
                ),
            )?,
        })
    }

    fn is_submarine(&self) -> bool {
        self.script.swap_type == SwapType::Submarine
    }
}

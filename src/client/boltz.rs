use boltz_client::swaps::boltzv2::{BoltzApiClientV2, CreateSubmarineRequest};
use pyo3::{pyclass, pymethods, PyResult};

use crate::types::client::SwapResponse;
use crate::types::submarine::CreateSubmarineResponse;
use crate::utils::errors::handle_rust_error;
use crate::utils::keys::parse_public_key;

#[pyclass]
pub struct Client {
    client: BoltzApiClientV2,
    referral_id: Option<String>,
}

#[pymethods]
impl Client {
    #[new]
    pub fn new(base_url: String, referral_id: Option<String>) -> Self {
        Client {
            referral_id,
            client: BoltzApiClientV2::new(&base_url),
        }
    }

    pub fn create_submarine_swap(
        &self,
        from: String,
        to: String,
        invoice: String,
        refund_public_key: Vec<u8>,
    ) -> PyResult<CreateSubmarineResponse> {
        let res = handle_rust_error(
            "could not create submarine swap",
            self.client.post_swap_req(&CreateSubmarineRequest {
                to,
                from,
                invoice,
                referral_id: self.referral_id.clone(),
                refund_public_key: parse_public_key(refund_public_key)?,
            }),
        )?;

        Ok(res.into())
    }


    pub fn get_pairs(&self) -> PyResult<SwapResponse> {
        let res = handle_rust_error(
            "could not fetch pairs",
            self.client.get_pairs()
        )?;

        Ok(res.into())
    }

}

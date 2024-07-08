use boltz_client::boltz::CreateSubmarineRequest;
use boltz_client::boltz::BoltzApiClientV2 as BoltzApiClient;
use pyo3::{pyclass, pymethods, PyResult};

use crate::types::client::{GetSubmarinePairsResponse, HeightResponse};
use crate::types::submarine::CreateSubmarineResponse;
use crate::utils::errors::handle_rust_error;
use crate::utils::keys::parse_public_key;

#[pyclass]
pub struct Client {
    client: BoltzApiClient,
    referral_id: Option<String>,
}

#[pymethods]
impl Client {
    #[new]
    pub fn new(base_url: String, referral_id: Option<String>) -> Self {
        Client {
            referral_id,
            client: BoltzApiClient::new(&base_url),
        }

    }

    pub fn create_submarine_swap(
        &self,
        from: String,
        to: String,
        invoice: String,
        refund_public_key: Vec<u8>,
        pair_hash: Option<String>,
    ) -> PyResult<CreateSubmarineResponse> {
        let res = handle_rust_error(
            "could not create submarine swap",
            self.client.post_swap_req(&CreateSubmarineRequest {
                to,
                from,
                invoice,
                pair_hash,
                referral_id: self.referral_id.clone(),
                refund_public_key: parse_public_key(refund_public_key)?,
            }),
        )?;

        Ok(res.into())
    }

    pub fn get_submarine_pairs(&self) -> PyResult<GetSubmarinePairsResponse> {
        let res = handle_rust_error("could not fetch pairs", self.client.get_submarine_pairs())?;

        Ok(res.into())
    }

    pub fn get_height(&self) -> PyResult<HeightResponse> {
        let res = handle_rust_error("could not fetch height", self.client.get_height())?;

        Ok(res.into())
    }
}

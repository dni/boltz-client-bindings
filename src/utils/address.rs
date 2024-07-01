use std::str::FromStr;

use boltz_client::elements;
use pyo3::{PyErr, pyfunction};
use pyo3::exceptions::PyValueError;

use crate::utils::errors::handle_rust_error;

const COULD_NOT_PARSE_ADDRESS: &str = "could not parse address";
const INVALID_NETWORK: &str = "invalid network";
const INVALID_CHAIN: &str = "invalid chain";

#[pyfunction]
pub fn validate_address(chain: String, network: String, address: String) -> Result<bool, PyErr> {
    match chain.as_str() {
        "BTC" => {
            let address = handle_rust_error(COULD_NOT_PARSE_ADDRESS, bitcoin::Address::from_str(address.as_str()))?;
            let network = match network.as_str() {
                "main" => bitcoin::Network::Bitcoin,
                "testnet" => bitcoin::Network::Testnet,
                "regtest" => bitcoin::Network::Regtest,
                _ => return Err(PyValueError::new_err(INVALID_NETWORK)),
            };
            Ok(address.is_valid_for_network(network))
        },
        "L-BTC" => {
            let address = handle_rust_error(COULD_NOT_PARSE_ADDRESS, elements::Address::from_str(address.as_str()))?;
            let network = match network.as_str() {
                "main" => elements::AddressParams::LIQUID,
                "testnet" => elements::AddressParams::LIQUID_TESTNET,
                "regtest" => elements::AddressParams::ELEMENTS,
                _ => return Err(PyValueError::new_err(INVALID_NETWORK)),
            };

            Ok(address.params == &network)
        },
        _ => Err(PyValueError::new_err(INVALID_CHAIN)),
    }
}

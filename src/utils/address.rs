use pyo3::pyfunction;


#[pyfunction]
pub fn validate_address(chain: String, network: String, address: String) -> bool {
    match chain.as_str() {
        "BTC" => {
            let address = bitcoin::Address::from_str(address)?;
            let network = match network.as_str() {
                "mainnet" => bitcoin::Network::Bitcoin,
                "testnet" => bitcoin::Network::Testnet,
                "regtest" => bitcoin::Network::Regtest,
                _ => return false,
            };
            address.is_valid_for_network(network)
        },
        "L-BTC" => {
            let address = elements::Address::from_str(address)?;
            let network = match network.as_str() {
                "mainnet" => elements::Network::Bitcoin,
                "testnet" => elements::Network::Testnet,
                "regtest" => elements::Network::Regtest,
                _ => return false,
            };
            address.is_valid_for_network(network)
        },
        _ => return false,
    }
}

use boltz_client::boltz::{SwapType, SwapTxKind};
use boltz_client::util::secrets::Preimage;
use boltz_client::Keypair;
use bitcoin::{Txid, Transaction};
use boltz_client::network::electrum::ElectrumConfig;
use std::convert::TryInto;
use std::str::FromStr;

use pyo3::{pyclass, pymethods, PyResult};
use pyo3::exceptions::PyValueError;

use crate::types::submarine::CreateSubmarineResponse;
use crate::types::reverse::CreateReverseResponse;
use crate::utils::errors::handle_rust_error;
use crate::utils::keys::parse_public_key;

#[pyclass]
#[derive(Debug, Clone)]
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

    #[staticmethod]
    fn from_reverse_response(
        created_response: CreateReverseResponse,
        claim_public_key: Vec<u8>,
    ) -> PyResult<Self> {
        Ok(BtcSwapScript {
            script: handle_rust_error(
                "could not parse response",
                boltz_client::BtcSwapScript::reverse_from_swap_resp(
                    &created_response.try_into()?,
                    parse_public_key(claim_public_key)?,
                ),
            )?,
        })
    }

    fn is_submarine(&self) -> bool {
        self.script.swap_type == SwapType::Submarine
    }

    fn is_reverse(&self) -> bool {
        self.script.swap_type == SwapType::ReverseSubmarine
    }
}


#[pyclass]
pub struct BtcSwapTx {
    #[pyo3(get)]
    pub kind: SwapTxKind,
    #[pyo3(get)]
    pub swap_script: BtcSwapScript,
    #[pyo3(get)]
    pub output_address: String,
    #[pyo3(get)]
    pub utxo: (String, u64),
}

#[pymethods]
impl BtcSwapTx {
    #[new]
    pub fn new(kind: SwapTxKind, swap_script: BtcSwapScript, output_address: String, utxo: (String, u64)) -> Self {
        BtcSwapTx {
            kind,
            swap_script,
            output_address,
            utxo,
        }
    }

    pub fn broadcast(&self, signed_tx: Vec<u8>, network_config: String) -> PyResult<String> {
        let network_config = ElectrumConfig::try_from(network_config.as_str()).map_err(|e| {
            PyValueError::new_err(format!("could not parse network config: {}", e))
        })?;
        let signed_tx = Transaction::from_signed_tx(signed_tx.as_slice()).map_err(|e| {
            PyValueError::new_err(format!("could not parse signed tx: {}", e))
        })?;
        let txid = handle_rust_error(
            "could not broadcast tx",
            self.swap_script.script.broadcast(&signed_tx, &network_config),
        )?;
        Ok(txid.to_string())
    }

    pub fn new_claim(&self, claim_address: String, network_config: String) -> PyResult<BtcSwapTx> {
        let network_config = ElectrumConfig::from_str(network_config.as_str()).map_err(|e| {
            PyValueError::new_err(format!("could not parse network config: {}", e))
        })?;
        let tx = handle_rust_error(
            "could not create claim tx",
            self.swap_script.script.new_claim(&claim_address, &network_config),
        )?;
        Ok(BtcSwapTx {
            kind: SwapTxKind::Claim,
            swap_script: self.swap_script.clone(),
            output_address: claim_address,
            utxo: self.utxo.clone(),
        })
    }

    pub fn new_refund(&self, refund_address: String, network_config: String) -> PyResult<BtcSwapTx> {
        let network_config = ElectrumConfig::from_str(network_config.as_str()).map_err(|e| {
            PyValueError::new_err(format!("could not parse network config: {}", e))
        })?;
        let tx = handle_rust_error(
            "could not create refund tx",
            self.swap_script.script.new_refund(&refund_address, &network_config),
        )?;
        Ok(BtcSwapTx {
            kind: SwapTxKind::Refund,
            swap_script: self.swap_script.clone(),
            output_address: refund_address,
            utxo: self.utxo.clone(),
        })
    }

    pub fn partial_sign(&self, keys: Vec<u8>, pub_nonce: String, transaction_hash: String) -> PyResult<(String, String)> {
        let keys = Keypair::from_slice(keys.as_slice()).map_err(|e| {
            PyValueError::new_err(format!("could not parse keys: {}", e))
        })?;
        let tx_hash = Txid::from_str(transaction_hash.as_str()).map_err(|e| {
            PyValueError::new_err(format!("could not parse tx hash: {}", e))
        })?;
        let (partial_sig, pub_nonce) = handle_rust_error(
            "could not sign tx",
            self.swap_script.script.partial_sign(&keys, &pub_nonce, &tx_hash),
        )?;
        Ok((partial_sig.to_string(), pub_nonce.to_string()))
    }

    pub fn sign_claim(&self, keys: Vec<u8>, preimage: Vec<u8>, absolute_fees: u64, is_cooperative: Option<bool>) -> PyResult<Vec<u8>> {
        let keys = Keypair::from_slice(keys.as_slice()).map_err(|e| {
            PyValueError::new_err(format!("could not parse keys: {}", e))
        })?;
        let preimage = Preimage::from_slice(preimage.as_slice()).map_err(|e| {
            PyValueError::new_err(format!("could not parse preimage: {}", e))
        })?;
        let tx = handle_rust_error(
            "could not sign claim tx",
            self.swap_script.script.sign_claim(&keys, &preimage, absolute_fees, is_cooperative),
        )?;
        Ok(tx.to_signed_tx().to_vec())
    }

    pub fn sign_refund(&self, keys: Vec<u8>, absolute_fees: u64, is_cooperative: Option<bool>) -> PyResult<Vec<u8>> {
        let keys = Keypair::from_slice(keys.as_slice()).map_err(|e| {
            PyValueError::new_err(format!("could not parse keys: {}", e))
        })?;
        let tx = handle_rust_error(
            "could not sign refund tx",
            self.swap_script.script.sign_refund(&keys, absolute_fees, is_cooperative),
        )?;
        Ok(tx.to_signed_tx().to_vec())
    }

    pub fn size(&self, keys: Vec<u8>, preimage: Vec<u8>) -> PyResult<usize> {
        let keys = Keypair::from_slice(keys.as_slice()).map_err(|e| {
            PyValueError::new_err(format!("could not parse keys: {}", e))
        })?;
        let preimage = Preimage::from_slice(preimage.as_slice()).map_err(|e| {
            PyValueError::new_err(format!("could not parse preimage: {}", e))
        })?;
        Ok(self.swap_script.script.size(&keys, &preimage))
    }

}

use bitcoin::{key::rand::thread_rng, secp256k1::Keypair};
use boltz_client::{PublicKey, ToHex};
use boltz_client::util::secrets::Preimage;
use boltz_client::network::Chain;
use pyo3::exceptions::PyValueError;
use pyo3::{pyfunction, PyErr};

use crate::utils::errors::to_python_error;

pub fn parse_public_key(public_key: Vec<u8>) -> Result<PublicKey, PyErr> {
    match PublicKey::from_slice(public_key.as_slice()) {
        Ok(k) => Ok(k),
        Err(err) => {
            return Err(to_python_error::<PyValueError, _>(
                "could not parse public key",
                err,
            ));
        }
    }
}

pub fn parse_preimage(preimage: Vec<u8>) -> Result<Preimage, PyErr> {
    let preimage_str = hex::encode(preimage);
    match Preimage::from_str(preimage_str.as_str()) {
        Ok(k) => Ok(k),
        Err(err) => {
            return Err(to_python_error::<PyValueError, _>(
                "could not parse preimage",
                err,
            ));
        }
    }
}

pub fn parse_chain(chain: String) -> Result<Chain, PyErr> {
    match chain.as_str() {
        "BTC" => {
            Ok(Chain::Bitcoin)
        },
        "L-BTC" => {
            Ok(Chain::Liquid)
        },
        _ => Err(PyValueError::new_err("invalid chain"))
    }
}

#[pyfunction]
pub fn new_keys() -> (Vec<u8>, Vec<u8>) {
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let keys = Keypair::new(&secp, &mut thread_rng());

    (
        keys.secret_key().as_ref().to_vec(),
        hex::decode(keys.public_key().to_hex()).expect("could not parse public key"),
    )
}

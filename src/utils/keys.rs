use bitcoin::{key::rand::thread_rng, secp256k1::Keypair};
use boltz_client::{PublicKey, ToHex};
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

pub fn parse_preimage_hash(preimage_hash: Vec<u8>) -> Result<[u8; 32], PyErr> {
    if preimage_hash.len() != 32 {
        return Err(to_python_error::<PyValueError, _>(
            "preimage hash must be 32 bytes long",
            "preimage hash must be 32 bytes long",
        ));
    }

    let mut hash = [0u8; 32];
    hash.copy_from_slice(&preimage_hash);

    Ok(hash)
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

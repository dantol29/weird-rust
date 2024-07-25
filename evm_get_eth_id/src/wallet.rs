use crate::ecdsa::EcdsaPublicKey;
use crate::state::lazy_call_ecdsa_public_key;
use candid::Principal;
use ic_ethereum_types::Address;
use serde_bytes::ByteBuf;

pub struct EthereumWallet {
    owner: Principal,                   // canister principal                
    derived_public_key: EcdsaPublicKey, // ethereum public key
}

impl EthereumWallet {
    pub async fn new(owner: Principal) -> Self {
        let derived_public_key = derive_public_key(&owner, &lazy_call_ecdsa_public_key().await); // get the ethereum public key
        Self { // return the EthereumWallet struct
            owner,
            derived_public_key,
        }
    }

    pub fn ethereum_address(&self) -> Address {
        Address::from(&self.derived_public_key)
    }
}

fn derive_public_key(owner: &Principal, public_key: &EcdsaPublicKey) -> EcdsaPublicKey {
    use ic_crypto_extended_bip32::{DerivationIndex, DerivationPath};
    let derivation_path = DerivationPath::new( // create a new derivation path - one of the steps in key creation
        derivation_path(owner)
            .into_iter() // convert the collection into an iterator.
            .map(DerivationIndex) // applies the DerivationIndex function to each item in the iterator.
            .collect(), // Collects the results of the map operation into a new collection
    );
    public_key
        .derive_new_public_key(&derivation_path)
        .expect("BUG: failed to derive an ECDSA public key")
}

// generates a collection (e.g., Vec<u32> or similar) of indices based on the owner parameter
fn derivation_path(owner: &Principal) -> Vec<Vec<u8>> {
    const SCHEMA_V1: u8 = 1;
    [
        ByteBuf::from(vec![SCHEMA_V1]),
        ByteBuf::from(owner.as_slice().to_vec()),
    ]
    .iter()
    .map(|x| x.to_vec())
    .collect()
}
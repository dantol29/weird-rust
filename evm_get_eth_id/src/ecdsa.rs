use ic_cdk::api::management_canister::ecdsa::EcdsaPublicKeyResponse;
use ic_crypto_ecdsa_secp256k1::PublicKey;
use ic_crypto_extended_bip32::{DerivationPath, ExtendedBip32DerivationResult};
use ic_ethereum_types::Address;
use ic_cdk::api::management_canister::ecdsa::{EcdsaKeyId, EcdsaCurve};
use std::cell::RefCell;

/* stores the ecdsa public key of the canister in a static variable
to maintain state across different calls to the canister */
thread_local! {
    static STATE: RefCell<Option<EcdsaPublicKey>> = RefCell::default();
}

/* get ECDSA public key from the IC managment canister,
ECDSA public key is used to derive the Ethereum address and is created 
by the IC management canister for a canister(not principal) */ 
pub async fn lazy_call_ecdsa_public_key() -> EcdsaPublicKey {
    use ic_cdk::api::management_canister::ecdsa::{ecdsa_public_key, EcdsaPublicKeyArgument};

    let key = STATE.with(|s| s.borrow().clone()); // get the public key from the state
    if let Some(ecdsa_pk) = key {
        return ecdsa_pk;    // return the public key if it already exists
    }

    let (response,) = ecdsa_public_key(EcdsaPublicKeyArgument { // send a request to the IC management canister to get ECDSA public key
        canister_id: None, // // defaults to the caller
        derivation_path: vec![],
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: String::from("dfx_test_key"),
        },
    })
    .await
    .unwrap_or_else(|(error_code, message)| { // if the request fails, trap with an error message
        ic_cdk::trap(&format!(
            "failed to get canister's public key: {} (error code = {:?})",
            message, error_code,
        ))
    });

    let pk = EcdsaPublicKey::from(response);
    STATE.with(|s| *s.borrow_mut() = Some(pk.clone())); // store the public key in the state
    pk // return the public key
}

// dfx_test_key: a default key ID that is used in deploying to a local version of IC (via IC SDK).
// test_key_1: a master test key ID that is used in mainnet.
// key_1: a master production key ID that is used in mainnet.


/* ECDSA public key struct that stores the public key and chain code */
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EcdsaPublicKey {
    public_key: PublicKey,
    chain_code: Vec<u8>,
}

/* deriving a new public key from an existing ECDSA public key 
basically lots of vryptographic operations that I do not understand*/
impl EcdsaPublicKey {
    pub fn derive_new_public_key(
        &self,
        derivation_path: &DerivationPath,
    ) -> ExtendedBip32DerivationResult<Self> {
        derivation_path
            .public_key_derivation(
                &self.public_key.serialize_sec1(/*compressed=*/ true),
                &self.chain_code,
            )
            .map(|output| Self {
                public_key: PublicKey::deserialize_sec1(&output.derived_public_key)
                    .expect("BUG: invalid public key"),
                chain_code: output.derived_chain_code,
            })
    }
}

impl AsRef<PublicKey> for EcdsaPublicKey {
    fn as_ref(&self) -> &PublicKey {
        &self.public_key
    }
}

impl From<EcdsaPublicKeyResponse> for EcdsaPublicKey {
    fn from(value: EcdsaPublicKeyResponse) -> Self {
        EcdsaPublicKey {
            public_key: PublicKey::deserialize_sec1(&value.public_key)
                .expect("BUG: invalid public key"),
            chain_code: value.chain_code,
        }
    }
}

impl From<&EcdsaPublicKey> for Address {
    fn from(value: &EcdsaPublicKey) -> Self {
        let key_bytes = value.as_ref().serialize_sec1(/*compressed=*/ false);
        debug_assert_eq!(key_bytes[0], 0x04);
        let hash = ic_crypto_sha3::Keccak256::hash(&key_bytes[1..]);
        let mut addr = [0u8; 20];
        addr[..].copy_from_slice(&hash[12..32]);
        Address::new(addr)
    }
}
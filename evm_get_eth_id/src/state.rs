use crate::ecdsa::EcdsaPublicKey;
use ic_cdk::api::management_canister::ecdsa::{EcdsaKeyId, EcdsaCurve};
use std::cell::RefCell;

/* stores the ecdsa public key of the canister in a static variable
to maintain state across different calls to the canister */
thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

/* get ECDSA public key from the IC managment canister,
ECDSA public key is used to derive the Ethereum address and is created 
by the IC management canister for a canister(not principal) */ 
pub async fn lazy_call_ecdsa_public_key() -> EcdsaPublicKey {
    use ic_cdk::api::management_canister::ecdsa::{ecdsa_public_key, EcdsaPublicKeyArgument};

    let key = STATE.with(|s| s.borrow().ecdsa_public_key.clone());
    if let Some(ecdsa_pk) = key {
        return ecdsa_pk;    // return the public key if it already exists
    }
    let key_id = STATE.with(|s| s.borrow().ecdsa_key_id().clone()); // get the key id from the state
    let (response,) = ecdsa_public_key(EcdsaPublicKeyArgument { // send a request to the IC management canister to get ECDSA public key
        canister_id: None,
        derivation_path: vec![],
        key_id,
    })
    .await
    .unwrap_or_else(|(error_code, message)| { // if the request fails, trap with an error message
        ic_cdk::trap(&format!(
            "failed to get canister's public key: {} (error code = {:?})",
            message, error_code,
        ))
    });
    let pk = EcdsaPublicKey::from(response);
    STATE.with(|s| s.borrow_mut().ecdsa_public_key = Some(pk.clone())); // store the public key in the state
    pk // return the public key
}

#[derive(Default)]
struct State {
    ecdsa_public_key: Option<EcdsaPublicKey>,
}

impl State {
    fn ecdsa_key_id(&self) -> EcdsaKeyId { // get the ECDSA key id
        EcdsaKeyId::from(EcdsaKeyId {curve: EcdsaCurve::Secp256k1, name: String::from("dfx_test_key")})
    }
}
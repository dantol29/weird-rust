mod wallet;
mod ecdsa;

use crate::wallet::EthereumWallet;
use candid::Principal;

#[ic_cdk::update]
async fn ethereum_address(owner: Option<Principal>) -> String {
    let caller = ic_cdk::caller();  // get principal of the caller
    let owner = owner.unwrap_or(caller); // unwrap_or method is used on Option and Result types(takes ownership)
                                         // to provide a default value in case the Option is None or the Result is Err
    let wallet = EthereumWallet::new(owner).await; // create or get a wallet
    wallet.ethereum_address().to_string() // return the ethereum address
}
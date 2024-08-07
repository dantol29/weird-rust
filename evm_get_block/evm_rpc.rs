#![allow(non_snake_case)]

use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::{self, api::call::CallResult};

pub const CANISTER_ID: Principal =    // EVM RPC canister id - defined in dfx.json to use with local replica
    Principal::from_slice(b"\x00\x00\x00\x00\x02\x30\x00\xCC\x01\x01"); // 7hfb6-caaaa-aaaar-qadga-cai

#[derive(CandidType, Deserialize)]
pub struct Block {                // struct returned by eth_getBlockByNumber call
    pub miner: String,
    pub totalDifficulty: u128,
    pub receiptsRoot: String,
    pub stateRoot: String,
    pub hash: String,
    pub difficulty: u128,
    pub size: u128,
    pub uncles: Vec<String>,
    pub baseFeePerGas: u128,
    pub extraData: String,
    pub transactionsRoot: Option<String>,
    pub sha3Uncles: String,
    pub nonce: u128,
    pub number: u128,
    pub timestamp: u128,
    pub transactions: Vec<String>,
    pub gasLimit: u128,
    pub logsBloom: String,
    pub parentHash: String,
    pub gasUsed: u128,
    pub mixHash: String,
}

#[derive(CandidType, Deserialize)]
pub enum BlockTag {   // specify a particular block in an Ethereum
    Earliest,     // Earliest: This refers to the earliest block (genesis block) in the blockchain.
    Safe,         // Safe: This refers to the latest block that is considered safe from reorganizations.
    Finalized,    // Finalized: This refers to the latest block that has been finalized and cannot be reorganized.
    Latest,       // Latest: This refers to the latest block in the blockchain.
    Number(u128), // Number(u128): This refers to a specific block by its block number.
    Pending,      // Pending: This refers to the pending block, i.e., the next block to be mined.
}

#[derive(CandidType, Deserialize)]
pub enum GetBlockByNumberResult {  // contains the actual result of the eth_getBlockByNumber call
    Ok(Block),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetBlockByNumberResult {                       // is used to handle the result of a eth_getBlockByNumber call to the EVM RPC canister.
    Consistent(GetBlockByNumberResult),                      // is used when the results from all queried services are consistent
    Inconsistent(Vec<(RpcService, GetBlockByNumberResult)>), // is used when the results from the queried services are inconsistent
}

#[derive(CandidType, Deserialize)]
pub enum EthMainnetService { // all avaliable mainnet services
    Alchemy,
    BlockPi,
    Cloudflare,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Deserialize)]
pub enum EthSepoliaService { // all avaliable testnet services
    Alchemy,
    BlockPi,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Deserialize)]
pub enum RpcServices {
    EthSepolia(Option<Vec<EthSepoliaService>>), // ETH testnet
    EthMainnet(Option<Vec<EthMainnetService>>), // ETH mainnet
}

pub struct EvmRpcCanister;
impl EvmRpcCanister {
    pub async fn eth_get_block_by_number(
        services: RpcServices,
        config: Option<RpcConfig>,
        block_tag: BlockTag,
        cycles: u128,
    ) -> CallResult<(MultiGetBlockByNumberResult,)> { // returns a tuple(right side is empty)
        ic_cdk::api::call::call_with_payment128(      // calls EVM RPC canister
            CANISTER_ID,
            "eth_getBlockByNumber",
            (services, config, block_tag),
            cycles,
        )
        .await
    }
}

#[derive(CandidType, Deserialize)]
pub enum Auth {
    RegisterProvider,
    FreeRpc,
    PriorityRpc,
    Manage,
}

#[derive(CandidType, Deserialize)]
pub struct HttpHeader {
    pub value: String,
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct RpcApi {
    pub url: String,
    pub headers: Option<Vec<HttpHeader>>,
}

#[derive(CandidType, Deserialize)]
pub struct RpcConfig {
    pub responseSizeEstimate: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct FeeHistoryArgs {
    pub blockCount: u128,
    pub newestBlock: BlockTag,
    pub rewardPercentiles: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct FeeHistory {
    pub reward: Vec<Vec<u128>>,
    pub gasUsedRatio: Vec<f64>,
    pub oldestBlock: u128,
    pub baseFeePerGas: Vec<u128>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ProviderError {
    TooFewCycles { expected: u128, received: u128 },
    MissingRequiredProvider,
    ProviderNotFound,
    NoPermission,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ValidationError {
    CredentialPathNotAllowed,
    HostNotAllowed(String),
    CredentialHeaderNotAllowed,
    UrlParseError(String),
    Custom(String),
    InvalidHex(String),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RejectionCode {
    NoError,
    CanisterError,
    SysTransient,
    DestinationInvalid,
    Unknown,
    SysFatal,
    CanisterReject,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum HttpOutcallError {
    IcError {
        code: RejectionCode,
        message: String,
    },
    InvalidHttpJsonRpcResponse {
        status: u16,
        body: String,
        parsingError: Option<String>,
    },
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RpcError {
    JsonRpcError(JsonRpcError),
    ProviderError(ProviderError),
    ValidationError(ValidationError),
    HttpOutcallError(HttpOutcallError),
}

#[derive(CandidType, Deserialize)]
pub enum FeeHistoryResult {
    Ok(Option<FeeHistory>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum RpcService {
    EthSepolia(EthSepoliaService),
    Custom(RpcApi),
    EthMainnet(EthMainnetService),
    Chain(u64),
    Provider(u64),
}
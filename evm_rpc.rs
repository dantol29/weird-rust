use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::{self, api::call::CallResult};

#[derive(CandidType, Deserialize)]
struct Block {                // struct returned by eth_getBlockByNumber call
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
enum BlockTag {   // specify a particular block in an Ethereum
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
enum EthMainnetService { // all avaliable mainnet services
    Alchemy,
    BlockPi,
    Cloudflare,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Deserialize)]
enum EthSepoliaService { // all avaliable testnet services
    Alchemy,
    BlockPi,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Deserialize)]
enum RpcServices {
    EthSepolia(Option<Vec<EthSepoliaService>>), // ETH testnet
    EthMainnet(Option<Vec<EthMainnetService>>), // ETH mainnet
}

struct EvmRpcCanister;
impl EvmRpcCanister {
    async fn eth_get_block_by_number(
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

#[ic_cdk::update]
async fn get_latest_eth_block() -> Block {
  let rpc_provider = RpcServices::EthMainnet(Some(vec![EthMainnetService::Cloudflare])); // choose ETH mainnet->cloudfare

  let cycles: u128 = 10_000_000_000;

  let (result,) =                               // expects a tuple. will save only the left side of the tuple.
        EvmRpcCanister::eth_get_block_by_number(rpc_providers, None, BlockTag::Latest, cycles) // calls a static function of struct EvmRpcCanister
            .await
            .expect("Call failed"); // If the function call results in an error, the program will panic and output the message "Call failed"
  
  match result {
        MultiGetBlockByNumberResult::Consistent(r) => match r {    // if the results from all queried services are consistent
            GetBlockByNumberResult::Ok(block) => block,            // return block info
            GetBlockByNumberResult::Err(err) => panic!("{err:?}"), // return an error
        },
        MultiGetBlockByNumberResult::Inconsistent(_) => {          // if the results from all queried services are inconsistent
            panic!("RPC providers gave inconsistent results")      // return an error
        }
    }
}




candid file:
  
type Block = record {
  baseFeePerGas : nat;
  difficulty : nat;
  extraData : text;
  gasLimit : nat;
  gasUsed : nat;
  hash : text;
  logsBloom : text;
  miner : text;
  mixHash : text;
  nonce : nat;
  number : nat;
  parentHash : text;
  receiptsRoot : text;
  sha3Uncles : text;
  size : nat;
  stateRoot : text;
  timestamp : nat;
  totalDifficulty : nat;
  transactions : vec text;
  transactionsRoot : opt text;
  uncles : vec text;
};

service : {
  /// Retrieve the latest block on the Ethereum blockchain.
  get_latest_ethereum_block : () -> (Block);
};

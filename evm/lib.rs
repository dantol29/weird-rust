mod evm_rpc;
use evm_rpc::{
    Block, BlockTag, EthMainnetService, EvmRpcCanister, GetBlockByNumberResult,
    MultiGetBlockByNumberResult, RpcServices,
};

#[ic_cdk::update]
async fn get_latest_eth_block() -> Block {
  let rpc_provider = RpcServices::EthMainnet(Some(vec![EthMainnetService::Cloudflare])); // choose ETH mainnet->cloudfare

  let cycles: u128 = 10_000_000_000;

  let (result,) =                               // expects a tuple. will save only the left side of the tuple.
        EvmRpcCanister::eth_get_block_by_number(rpc_provider, None, BlockTag::Latest, cycles) // calls a static function of struct EvmRpcCanister
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

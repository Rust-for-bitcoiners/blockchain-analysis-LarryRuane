use std::{env, time};

use bitcoincore_rpc::{
    json,
    jsonrpc::{self},
    Auth, Client, RpcApi,
    bitcoin,
};
use chrono::{Duration, TimeDelta};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RPC_CLIENT: Client = {
        dotenv::dotenv().ok();
        let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password: String =
            env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).unwrap()
    };
}

// static client: Client = Client::new("url", Auth::UserPass("user".to_owned(), "password".to_owned())).unwrap();

fn get_block(height: u64) -> bitcoin::Block {
    // * is a deref operator which invokes the Deref trait of the type RPC_CLIENT which was created
    // when the lazy macro is expanded
    // if a value has a static lifetime then it means that value lives as long as the program lives
    let rpc_client: &Client = &*RPC_CLIENT;

    let hash: bitcoin::BlockHash = rpc_client.get_block_hash(height).unwrap();
    rpc_client.get_block(&hash).unwrap()
}
// TODO: Task 1
fn time_to_mine(block_height: u64) -> Duration {
    let block = get_block(block_height);
    let prev_block = get_block(block_height - 1);

    let block_time = TimeDelta::try_seconds(block.header.time as i64).unwrap();
    let prev_block_time = TimeDelta::try_seconds(prev_block.header.time as i64).unwrap();

    block_time.checked_sub(&prev_block_time).unwrap()
}

// TODO: Task 2
fn number_of_transactions(block_height: u64) -> usize {
    let block = get_block(block_height);
    block.txdata.len()
}

fn main() {
    // you can use rpc_client here as if it was a global variable
    // println!("{:?}", res);
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} block_height", args[0]);
        std::process::exit(1);
    }
    let height:u64 = args[1].to_string().parse().unwrap();
    const TIMEOUT_UTXO_SET_SCANS: time::Duration = time::Duration::from_secs(60 * 8); // 8 minutes
    dotenv::dotenv().ok();
        let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password: String =
            env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");

    let custom_timeout_transport = jsonrpc::simple_http::Builder::new()
        .url(&rpc_url)
        .expect("invalid rpc url")
        .auth(rpc_user, Some(rpc_password))
        .timeout(TIMEOUT_UTXO_SET_SCANS)
        .build();
    let custom_timeout_rpc_client =
        jsonrpc::client::Client::with_transport(custom_timeout_transport);

    let rpc_client = Client::from_jsonrpc(custom_timeout_rpc_client);
    let res: json::GetBlockchainInfoResult =
        rpc_client.get_blockchain_info().unwrap();
    println!("{:?}", res);

    println!("block {}: time_to_mine: {}", height, time_to_mine(height).num_seconds());
    println!("block {}: number of transactions: {}", height, number_of_transactions(height));
}

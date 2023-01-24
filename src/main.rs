mod balance;
mod coingecko;
mod erc_20;
use crate::{balance::TokenBalance, utils::create_csv_file};
use coingecko::get_tokens_information;
use itertools::Itertools;
use log::{error, info};
mod utils;

/// This is the address of the wallet you want to check
const YOUR_ADDRESS:&str = "0xtherestofyouraddress";
/// This is the blockchain you want to check, for example ethereum, binance, polygon, etc, only EVMs are supported
const BLOCKCHAIN:&str = "ethereum";
/// This is the address of the node you want to connect to, ideally it is a paid non-throttled node, because
/// we will be sending thousands of concurrent requests
const NODE_ADDRESS:&str = "https://eth-mainnet.alchemyapi.io/v2/your_api_key";

#[tokio::main]
async fn main() {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    // first we get all the known tokens from coingecko
    match get_tokens_information().await {
        Ok(tokens) => {
            info!("Success obtaining token data");
            let tokens = tokens
                .iter()
                .filter(|t| t.platforms.contains_key(BLOCKCHAIN))
                .collect_vec();
            info!("Found {} tokens with {BLOCKCHAIN} platform", tokens.len());
            // we set up queries at each one, then execute them all at once
            let tasks: Vec<_> = tokens
                .iter()
                .map(|t| {
                    TokenBalance::get(
                        YOUR_ADDRESS,
                        t.platforms[BLOCKCHAIN].as_ref().unwrap().to_string(),
                    )
                })
                .collect_vec();
            let results = futures::future::join_all(tasks).await;
            let tokens_with_balances = results.iter().filter_map(|t| t.as_ref()).collect_vec();
            create_csv_file(&tokens_with_balances, "balances");
        }
        Err(e) => error!("Could not get token info from coingecko. Error: {:?}", e),
    }
}

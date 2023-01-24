use crate::erc_20::ERC_20_ABI;
use crate::utils::get_timestamp_string;
use crate::utils::get_token_balance_link;
use crate::NODE_ADDRESS;
use log::error;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::Address;


/// The `TokenBalance` struct contains everything we would need to know about a token balance for reporting.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TokenBalance {
    pub symbol: String,
    pub balance: f64,
    pub name: String,
    pub token_address: String,
    pub timestamp: String,
    pub link: String,
    pub blockchain: String,
}
impl TokenBalance {
    pub async fn get(wallet_address: &str, token_address: String) -> Option<TokenBalance> {
        match web3::transports::Http::new(NODE_ADDRESS) {
            Ok(w3) => {
                let web3_instance = web3::Web3::new(w3.clone());
                if let Ok(parsed_token_address) = Address::from_str(&token_address) {
                    match Contract::from_json(
                        web3_instance.eth(),
                        parsed_token_address,
                        ERC_20_ABI.as_bytes(),
                    ) {
                        Ok(contract) => {
                            let mut initial = TokenBalance::default();
                            if let Ok(name) = contract
                                .query("name", (), None, Options::default(), None)
                                .await
                            {
                                initial.name = name;
                                if let Ok(balance) = contract
                                    .query(
                                        "balanceOf",
                                        (Address::from_str(&wallet_address).unwrap(),),
                                        None,
                                        Options::default(),
                                        None,
                                    )
                                    .await
                                {
                                    if let Ok(decimals) = contract
                                        .query("decimals", (), None, Options::default(), None)
                                        .await
                                    {
                                        let _: i64 = balance;
                                        let _: i32 = decimals;
                                        let divider = 10_f64.powi(decimals);
                                        initial.balance = balance as f64 / divider;
                                        if let Ok(symbol) = contract
                                            .query("symbol", (), None, Options::default(), None)
                                            .await
                                        {
                                            initial.symbol = symbol;
                                            initial.token_address = token_address.clone();
                                            initial.timestamp = get_timestamp_string();
                                            initial.link = get_token_balance_link(
                                                &token_address,
                                                &wallet_address,
                                            );
                                            return Some(initial);
                                        }
                                    }
                                }
                            }
                            error!("Could not get token name for {}", token_address);
                            return None;
                        }
                        Err(e) => {
                            error!("{:?}", e);
                            return None;
                        }
                    }
                }
                error!("Could not parse token address: {}", token_address);
                None
            }
            Err(e) => {
                error!("{:?}", e);
                None
            }
        }
    }
}

use std::collections::HashMap;
use serde::Serialize;
use serde::Deserialize;



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TokenInformation {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub platforms: HashMap<String,Option<String>>
}

/// Gets all token data from coingecko
/// `https://api.coingecko.com/api/v3/coins/list?include_platform=true`
pub async fn get_tokens_information() -> Result<Vec<TokenInformation>, reqwest::Error> {
    let url = "https://api.coingecko.com/api/v3/coins/list?include_platform=true";
    return reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json::<Vec<TokenInformation>>()
        .await
}


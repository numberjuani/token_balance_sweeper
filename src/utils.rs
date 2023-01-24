use csv::WriterBuilder;

pub fn get_timestamp_string() -> String {
    return chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
}

pub fn get_token_balance_link(token_address:&str,wallet_address:&str) -> String {
    format!("https://etherscan.io/token/{}?a={}",token_address,wallet_address)
}

pub fn create_csv_file<T: serde::Serialize>(data: &[T], filename: &str) {
    let filename = format!(
        "{}-{}.csv",
        filename,
        chrono::Local::now().format("%F-%H%M")
    );
    if !data.is_empty() {
        let mut writer = WriterBuilder::new().has_headers(true).from_path(filename).unwrap();
        for line in data {
            writer.serialize(line).unwrap();
        }
    }
}
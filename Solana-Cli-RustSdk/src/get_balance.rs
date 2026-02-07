use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use anyhow::{Result, Context};

pub async fn get_balance(address : String)-> Result<u64> {
    let connection = RpcClient::new("https://api.devnet.solana.com".to_string());

    let pub_address = Pubkey::from_str(&address)
        .context("Invalid Solana address")?;

    let balance = connection.get_balance(&pub_address).await?;
    Ok(balance)
}
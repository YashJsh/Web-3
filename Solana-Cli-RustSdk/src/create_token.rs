use solana_client::nonblocking::rpc_client;
use solana_sdk::program_pack::Pack;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use solana_system_interface::{instruction::create_account};
use spl_token_2022::{instruction::initialize_mint};
use spl_token_2022::{ID as token_progam_id};

struct MetaData{
    name : String,
    symbol : String,
    uri : String,
}

async fn create_mint(key: String, name : String, symbol : String, uri : String)-> anyhow::Result<String>{
    let connection = rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());
    let keypair = Keypair::new();
    
    let data_len = spl_token_2022::state::Mint::LEN;
    let min_lamports = connection
        .get_minimum_balance_for_rent_exemption(data_len)
        .await?;
    let payer = Keypair::from_base58_string(&key);
    let metadata = MetaData{
        name,
        symbol,
        uri
    };

    let create_account = create_account(
        &payer.pubkey(),
        &keypair.pubkey(),
        min_lamports,
        data_len as u64,
        &token_progam_id,
    );
    let create_mint =initialize_mint(
        &token_progam_id, 
        &keypair.pubkey(), 
        &payer.pubkey(), 
        Some(&payer.pubkey()), 
        9
    )?;
    let mut txn = Transaction::new_with_payer(&[create_account, create_mint],Some(&payer.pubkey()));
    let blockhash = connection.get_latest_blockhash().await?;
    txn.sign(&[&payer, &keypair], blockhash);
    let sign = connection.send_and_confirm_transaction(&txn).await?;

    println!("Mint Address is : {}", keypair.pubkey());
    Ok(sign.to_string())
}

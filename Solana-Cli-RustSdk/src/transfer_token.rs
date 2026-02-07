use solana_client::{client_error::ClientError, nonblocking::rpc_client::RpcClient};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use std::str::FromStr;
use solana_system_interface::instruction::transfer;

pub async fn send_sol(
    private_key: String,
    amount: u32,
    receiver_address: String,
) -> Result<Signature, ClientError> {
    let connection = RpcClient::new("https://api.devnet.solana.com".to_string());

    let keypair = Keypair::from_base58_string(&private_key);

    let recipient_pubkey = Pubkey::from_str(&receiver_address).unwrap();

    let instruction = transfer(
        &keypair.pubkey(),
        &recipient_pubkey,
        amount as u64 * LAMPORTS_PER_SOL,
    );

    let blockhash = connection.get_latest_blockhash().await?;

    let mut transaction =
        Transaction::new_with_payer(&[instruction], Some(&keypair.pubkey()));

    transaction.sign(&[&keypair], blockhash);

    let signature = connection
        .send_and_confirm_transaction(&transaction)
        .await?;

    Ok(signature)
}

use solana_client::nonblocking::rpc_client;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use solana_system_interface::instruction::create_account;
use spl_token_2022::ID as token_progam_id;
use spl_token_2022::extension::ExtensionType;
use spl_token_2022::instruction::initialize_mint;
use spl_token_2022_interface::extension::metadata_pointer::instruction::initialize as initialize_metadata_pointer;
use spl_token_2022_interface::state::Mint;
use spl_token_metadata_interface::instruction::initialize as intialize_token_metadata;
use spl_token_metadata_interface::state::TokenMetadata;

pub struct MintResult {
    pub signature: String,
    pub mint: String,
}

pub async fn create_mint(
    key: String,
    name: String,
    symbol: String,
    uri: String,
) -> anyhow::Result<MintResult> {
    let connection = rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());
    println!("> Creating a new key pair for the mint account");
    let mint = Keypair::new();

    let payer = Keypair::from_base58_string(&key);
    let metadata = TokenMetadata {
        update_authority: Some(payer.pubkey()).try_into()?,
        mint: mint.pubkey(),
        name,
        symbol,
        uri,
        additional_metadata: vec![],
    };
    println!("> Calculating the space for the extension");
    let mint_space =
        ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::MetadataPointer])?;

    println!(
        "> Calculating the lamports required for the minimum rent exemption for token account"
    );
    let meta_dat_len = metadata.tlv_size_of()?;
    let min_lamports = connection
        .get_minimum_balance_for_rent_exemption(mint_space + meta_dat_len)
        .await?;

    let create_account = create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        min_lamports,
        mint_space as u64,
        &token_progam_id,
    );
    let initialize_meta_data_pointer_instruction = initialize_metadata_pointer(
        &token_progam_id,
        &mint.pubkey(),
        Some(payer.pubkey()),
        Some(mint.pubkey()),
    )?;

    let create_mint = initialize_mint(
        &token_progam_id,
        &mint.pubkey(),
        &payer.pubkey(),
        Some(&payer.pubkey()),
        9,
    )?;

    let intialize_meta_data_instruction = intialize_token_metadata(
        &token_progam_id,
        &mint.pubkey(),
        &payer.pubkey(),
        &mint.pubkey(),
        &payer.pubkey(),
        metadata.name,
        metadata.symbol,
        metadata.uri,
    );
    println!("> Creating transaction with all the instructions");

    let mut txn = Transaction::new_with_payer(
        &[
            create_account,
            initialize_meta_data_pointer_instruction,
            create_mint,
            intialize_meta_data_instruction,
        ],
        Some(&payer.pubkey()),
    );
    let blockhash = connection.get_latest_blockhash().await?;
    txn.sign(&[&payer, &mint], blockhash);
    let sign = connection.send_and_confirm_transaction(&txn).await?;

    println!("Mint Address is : {}", mint.pubkey());
    Ok(MintResult {
        mint: mint.pubkey().to_string(),
        signature: sign.to_string(),
    })
}

use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    commitment_config::CommitmentConfig,
};
use solana_sdk::transaction::Transaction;
use spl_token::{instruction::initialize_mint, state::Mint};
use std::env;
use solana_sdk::message::Message;
use solana_sdk::transaction::TransactionError;
use solana_sdk::signer::Signer as _;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    
    let secret_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let keypair_bytes: Vec<u8> = serde_json::from_str(&secret_key)?;
    let keypair = Keypair::from_bytes(&keypair_bytes)?;

    // Set up Solana connection
    let rpc_url = solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());

    // Print public key
    println!("🔑 Our public key is: {}", keypair.pubkey());

    // Create Mint (This is just a placeholder - you'll need to use the spl-token crate correctly)
    let mint = create_mint(&rpc_url, &keypair)?;

    // Construct token mint link (You'll need to create a function to generate this)
    let link = format!("https://explorer.solana.com/address/{}?cluster=devnet", mint);

    println!("✅ Token Mint: {}", link);

    Ok(())
}

fn create_mint(
    client: &RpcClient,
    payer: &Keypair,
) -> Result<Pubkey, TransactionError> {
    Ok(Pubkey::new_unique())
}

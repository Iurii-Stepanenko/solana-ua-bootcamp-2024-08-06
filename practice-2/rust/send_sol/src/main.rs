use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::env;
use std::error::Error;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let secret_key = env::var("SECRET_KEY")?;
    let keypair_bytes: Vec<u8> = serde_json::from_str(&secret_key)?;
    let sender = Keypair::from_bytes(&keypair_bytes)?;

    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    println!("🔑 Our public key is: {}", sender.pubkey());

    let recipient = Pubkey::from_str("4SWLvY9Kq8YTZycUGHVgm8bySvWKF8F6CEM7iVGifxq5")?;
    println!("💸 Attempting to send 0.01 SOL to {}...", recipient);

    let lamports = 0.01 * solana_sdk::native_token::LAMPORTS_PER_SOL as f64;
    let transfer_instruction = system_instruction::transfer(
        &sender.pubkey(),
        &recipient,
        lamports as u64,
    );

    // Create and send transaction with transfer instruction
    let mut transaction = Transaction::new_with_payer(&[transfer_instruction.clone()], Some(&sender.pubkey()));
    let recent_blockhash = client.get_latest_blockhash()?;
    transaction.sign(&[&sender], recent_blockhash);

    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("✅ Transaction confirmed, signature: {}", signature);

    // Create memo instruction
    let memo_program = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")?;
    let memo_text = "Hello from Iurii Stepanenko!";

    let memo_instruction = Instruction {
        program_id: memo_program,
        accounts: vec![solana_sdk::instruction::AccountMeta::new(sender.pubkey(), true)],
        data: memo_text.as_bytes().to_vec(),
    };

    // Create and send a new transaction with both transfer and memo instructions
    let mut transaction_with_memo = Transaction::new_with_payer(
        &[transfer_instruction, memo_instruction],
        Some(&sender.pubkey())
    );
    transaction_with_memo.sign(&[&sender], recent_blockhash);

    let signature_with_memo = client.send_and_confirm_transaction(&transaction_with_memo)?;
    println!("📝 Memo sent with signature: {}", signature_with_memo);

    Ok(())
}

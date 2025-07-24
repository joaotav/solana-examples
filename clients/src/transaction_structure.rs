use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::Signer,
    signer::keypair::Keypair, system_instruction, transaction::Transaction,
};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = RpcClient::new_with_commitment(
        "http://localhost:8899".to_string(),
        CommitmentConfig::confirmed(),
    );

    // Fetch the latest blockhash and last valid block height
    let blockhash = connection.get_latest_blockhash().await?;

    // Generate sender and recipient keypairs
    let sender = Keypair::new();
    let recipient = Keypair::new();

    // Create a transfer instruction for transferring SOL from sender to recipient
    let transfer_instruction = system_instruction::transfer(
        &sender.pubkey(),
        &recipient.pubkey(),
        LAMPORTS_PER_SOL / 100, // 0.01 SOL
    );

    let mut transaction =
        Transaction::new_with_payer(&[transfer_instruction], Some(&sender.pubkey()));
    transaction.sign(&[&sender], blockhash);

    println!("{:#?}", transaction);

    Ok(())
}

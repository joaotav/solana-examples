use anyhow::Result;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, signature::Signer, signer::keypair::Keypair, system_instruction,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Generate sender and recipient keypairs
    let sender = Keypair::new();
    let recipient = Keypair::new();

    // Define the amount to transfer
    let transfer_amount = LAMPORTS_PER_SOL / 100; // 0.01 SOL

    // Create a transfer instruction for transferring SOL from sender to recipient
    let transfer_instruction =
        system_instruction::transfer(&sender.pubkey(), &recipient.pubkey(), transfer_amount);

    println!("{:#?}", transfer_instruction);

    Ok(())
}

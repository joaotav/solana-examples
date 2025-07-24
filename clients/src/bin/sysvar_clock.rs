use anyhow::Result;
use bincode::deserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    sysvar::{self, clock::Clock},
};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = RpcClient::new_with_commitment(
        "https://api.mainnet-beta.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let account = connection.get_account(&sysvar::clock::ID).await?;
    // Deserialize the account data
    let clock: Clock = deserialize(&account.data)?;

    println!("{:#?}", account);
    println!("{:#?}", clock);

    Ok(())
}

use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let program_id = Pubkey::from_str("11111111111111111111111111111111")?;
    let optional_seed = b"helloWorld";

    // Loop through all bump seeds (255 down to 0)
    for bump in (0..=255).rev() {
        match Pubkey::create_program_address(&[optional_seed.as_ref(), &[bump]], &program_id) {
            Ok(pda) => println!("bump {}: {}", bump, pda),
            Err(err) => println!("bump {}: {}", bump, err),
        }
    }

    Ok(())
}

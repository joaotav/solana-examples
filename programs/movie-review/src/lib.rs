use anchor_lang::prelude::*;

declare_id!("7yNXF6wFHjHe2QANXwF9qFVyhUW6Th4NAEPzTgqNepcu");

#[program]
pub mod movie_review {
    use super::*;
}

#[account]
#[derive(InitSpace)]
pub struct MovieAccountState {
    pub review: Pubkey, // 32 bytes
    pub rating: u8,     // 1 byte
    #[max_len(80)]
    pub title: String, // 4 + len()
    #[max_len(200)]
    pub description: String, // 4 + len()
}

const DISCRIMINATOR: usize = 8;

#[error_code]
enum MovieReviewError {
    #[msg("Rating must be between 1 and 10")]
    InvalidRating,
    #[msg("Movie title too long")]
    TitleTooLong,
    #[msg("Description too long")]
    DescriptionTooLong,
}

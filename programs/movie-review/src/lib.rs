use anchor_lang::prelude::*;

declare_id!("7yNXF6wFHjHe2QANXwF9qFVyhUW6Th4NAEPzTgqNepcu");

const MIN_RATING: u8 = 1;
const MAX_RATING: u8 = 10;
const MAX_TITLE_LENGTH: usize = 80;
const MAX_DESCRIPTION_LENGTH: usize = 200;
const DISCRIMINATOR: usize = 8;

#[program]
pub mod movie_review {
    use super::*;

    pub fn add_movie_review(
        ctx: Context<MovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        require!(
            rating >= MIN_RATING && rating <= MAX_RATING,
            MovieReviewError::InvalidRating
        );

        require!(
            title.len() <= MAX_TITLE_LENGTH,
            MovieReviewError::TitleTooLong
        );

        require!(
            description.len() <= MAX_DESCRIPTION_LENGTH,
            MovieReviewError::DescriptionTooLong
        );

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.description = description;
        movie_review.rating = rating;

        msg!("Movie review account created.");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct MovieAccountState {
    pub review: Pubkey, // 32 bytes
    #[max_len(MAX_TITLE_LENGTH)]
    pub title: String, // 4 + len()
    #[max_len(MAX_DESCRIPTION_LENGTH)]
    pub description: String, // 4 + len()
    pub rating: u8,     // 1 byte
}

#[error_code]
enum MovieReviewError {
    #[msg("Rating must be between 1 and 10")]
    InvalidRating,
    #[msg("Movie title too long")]
    TitleTooLong,
    #[msg("Description too long")]
    DescriptionTooLong,
}

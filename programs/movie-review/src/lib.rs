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
        movie_review.reviewer = ctx.accounts.reviewer.key();
        movie_review.title = title;
        movie_review.description = description;
        movie_review.rating = rating;

        msg!("Movie review account created.");
        msg!("Title: {}", movie_review.title);
        msg!("Description: {}", movie_review.description);
        msg!("Rating: {}", movie_review.rating);

        Ok(())
    }

    pub fn update_movie_review(
        ctx: Context<UpdateMovieReview>, 
        title: String, 
        description: String, 
        rating: u8
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
        movie_review.description = description;
        movie_review.rating = rating;

        msg!("Movie review updated.");
        msg!("Title: {}", movie_review.title);
        msg!("Description: {}", movie_review.description);
        msg!("Rating: {}", movie_review.rating);

        Ok(())
    }

    pub fn delete_movie_review(_ctx: Context<DeleteMovieReview>, title: String) -> Result<()> {
        // Because we are closing the account we don't need any instruction logic inside the body
        // of the function. The account closing will be handled by the 'close' constraint in the
        // DeleteMovieReview struct.
        msg!("Movie review for {} deleted", title);
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct AddMovieReview<'info> {
    #[account(
        init, 
        seeds = [title.as_bytes(), reviewer.key().as_ref()], 
        bump, 
        payer = reviewer, 
        space = DISCRIMINATOR + MovieReviewData::INIT_SPACE,
    )]
    pub movie_review: Account<'info, MovieReviewData>,
    #[account(mut)]
    pub reviewer: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = DISCRIMINATOR + MovieReviewData::INIT_SPACE,
        // Set the reviewer as the payer for any additional lamports required for rent 
        // exemption in case the new description is larger than the previous one.
        realloc::payer = reviewer,
        realloc::zero = true,
    )]
    pub movie_review: Account<'info, MovieAcccountState>,
    #[account(mut)]
    reviewer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct MovieReviewData {
    pub reviewer: Pubkey, // 32 bytes
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

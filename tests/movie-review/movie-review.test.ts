import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { MovieReviewProgram } from "../../target/types/movie_review_program"

describe("movie-review-program", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.MovieReviewProgram as Program<MovieReviewProgram>;
  
  const movie = {
    title: "Test movie",
    description: "Such good movie, really great, very amazing",
    rating: 10,
  }
  
  const [moviePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(movie.title), provider.wallet.publicKey.toBuffer()],
    program.programId
  );
  
  it("Movie review can be added", async () => {
    const tx = await program.methods
      .addMovieReview(movie.title, movie.description, movie.rating)
      .rpc();
      
    const account = await program.account.movieReviewData.fetch(moviePda);
    
    // Use .equals() for comparing strings or .to.equal for direct comparison
    expect(account.title).to.equal(movie.title);
    expect(account.description).to.equal(movie.description);
    expect(account.rating).to.equal(movie.rating);
    
    // For PublicKey comparison, use .equals() method or convert to string
    expect(account.reviewer.equals(provider.wallet.publicKey)).to.be.true;
    // OR
    // expect(account.reviewer.toString()).to.equal(provider.wallet.publicKey.toString());
  });
  
  it("Movie review can be updated", async () => {
    const newDescription = "Wow this is new";
    const newRating = 4;
    
    const tx = await program.methods
      .updateMovieReview(movie.title, newDescription, newRating)
      .rpc();
      
    const account = await program.account.movieReviewData.fetch(moviePda);
    
    expect(account.title).to.equal(movie.title);
    expect(account.description).to.equal(newDescription);
    expect(account.rating).to.equal(newRating);
    expect(account.reviewer.equals(provider.wallet.publicKey)).to.be.true;
  });
  
  it("Movie review can be deleted", async () => {
    const tx = await program.methods.deleteMovieReview(movie.title).rpc();
    
    // Optionally, verify the account no longer exists
    try {
      await program.account.movieReviewData.fetch(moviePda);
      expect.fail("Account should have been deleted");
    } catch (error) {
      expect(error.message).to.include("Account does not exist");
    }
  });
});
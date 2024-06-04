import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OnchainCommunity } from "../target/types/onchain_community";

describe("onchain-community", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.OnchainCommunity as Program<OnchainCommunity>;

  const comment1Keypair = anchor.web3.Keypair.generate();

  it("Create commment", async () => {
    try {
      const tx = await program.methods
      .createComment(
        "https://n.news.naver.com/mnews/article/654/0000076640",
        "this is first comment :)"
      ).accounts({
        comment: comment1Keypair.publicKey,
        authority: provider.wallet.publicKey,
      }).signers([comment1Keypair])
      .rpc();
    } catch (err) {
      console.log(err);
    }
    const comment = await program.account.commentAccount.all();
    console.log(comment);
  });
});

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OnchainCommunity } from "../target/types/onchain_community";

describe("onchain-community", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.OnchainCommunity as Program<OnchainCommunity>;

  const comment1Keypair = anchor.web3.Keypair.generate();

  const decodeUTF8Array = (data) => {
    return new TextDecoder('utf-8').decode(new Uint8Array(data));
  };

  it("get data", async () => {
    const comment = await program.account.commentAccount.all();
    for (const item of comment) {
      const url = decodeUTF8Array(item.account.url);
      console.log("URL:", url);
      console.log(item.account.authority.toBase58())
    }
  })

  // it("Create commment", async () => {
  //   try {
  //     const tx = await program.methods
  //     .createComment(
  //       "edition.cnn.com/2024/06/17/politics/us-alarm-china-north-korea-russia",
  //       "Can we get more information on this? It's quite intriguing."
  //     ).accounts({
  //       comment: comment1Keypair.publicKey,
  //       authority: provider.wallet.publicKey,
  //     }).signers([comment1Keypair])
  //     .rpc();
  //   } catch (err) {
  //     console.log(err);
  //   }
  //   const comment = await program.account.commentAccount.all();

  //   const url = decodeUTF8Array(comment[0].account.url)
  //   const content = decodeUTF8Array(comment[0].account.content)
  //   console.log(url)
  //   console.log(content)
  // });
});

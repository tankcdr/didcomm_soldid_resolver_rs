import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { DidSolIdentifier, DidSolService } from "@identity.com/sol-did-client";
import { TestAnchorWallet } from "./TestAnchorWallet";
import { loadKeypair } from "./utils";

async function cleanup() {
  const connection = new Connection(
    "http://api.devnet.solana.com",
    "confirmed"
  );

  try {
    const authority = loadKeypair("./tests/data/test-account.json");
    const anchorWallet = new TestAnchorWallet(authority);

    const didIdentifier = DidSolIdentifier.create(
      authority.publicKey,
      "devnet"
    );

    const service = await DidSolService.build(didIdentifier, {
      connection,
      wallet: anchorWallet,
    });

    let result = await service.close(new PublicKey(authority.publicKey)).rpc();

    console.log("Cleanup completed successfully", result);
  } catch (error) {
    console.error("Error during cleanup:", error);
    process.exit(1);
  }
}

cleanup();

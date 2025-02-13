import { Connection } from "@solana/web3.js";
import { DidSolIdentifier, DidSolService } from "@identity.com/sol-did-client";
import { TestAnchorWallet } from "./TestAnchorWallet";
import { loadKeypair } from "./utils";

async function addService() {
  const connection = new Connection(
    "http://api.devnet.solana.com",
    "finalized"
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

    let transaction = await service
      .addService({
        fragment: "agent",
        serviceType: "TestService",
        serviceEndpoint: "https://test-service.com",
      })
      .withAutomaticAlloc(authority.publicKey)
      .rpc();

    console.log("Transaction:", transaction);
    await connection.confirmTransaction(transaction, "finalized");

    const did_doc = await service.resolve();
    console.log("DID Document after adding verification method:", did_doc);

    console.log("addService completed successfully");
  } catch (error) {
    console.error("Error during addService:", error);
    process.exit(1);
  }
}

addService();

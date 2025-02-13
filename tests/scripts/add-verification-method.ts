import { Connection } from "@solana/web3.js";
import {
  BitwiseVerificationMethodFlag,
  DidSolIdentifier,
  DidSolService,
  VerificationMethodType,
} from "@identity.com/sol-did-client";
import { TestAnchorWallet } from "./TestAnchorWallet";
import { loadKeypair } from "./utils";

async function addVerificationMethod() {
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
      .addVerificationMethod({
        fragment: "key-2",
        keyData: authority.publicKey.toBuffer(),
        methodType: VerificationMethodType.Ed25519VerificationKey2018,
        flags: [
          BitwiseVerificationMethodFlag.Assertion,
          BitwiseVerificationMethodFlag.KeyAgreement,
        ],
      })
      .withAutomaticAlloc(authority.publicKey)
      .rpc();

    console.log("Transaction:", transaction);
    await connection.confirmTransaction(transaction, "finalized");

    const did_doc = await service.resolve();
    console.log("DID Document after adding verification method:", did_doc);

    console.log("addVerificationMethod completed successfully");
  } catch (error) {
    console.error("Error during addVerificationMethod:", error);
    process.exit(1);
  }
}

addVerificationMethod();

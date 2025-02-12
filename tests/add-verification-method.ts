import { Connection, Keypair } from "@solana/web3.js";
import {
  BitwiseVerificationMethodFlag,
  DidSolIdentifier,
  DidSolService,
  VerificationMethodType,
} from "@identity.com/sol-did-client";
import fs from "fs";
import { TestAnchorWallet } from "./TestAnchorWallet";

async function addVerificationMethod() {
  try {
    // Read the test data
    const testData = JSON.parse(fs.readFileSync(process.argv[2], "utf-8"));
    const keypair = Keypair.fromSecretKey(new Uint8Array(testData.keypair));

    // Create Anchor wallet
    const anchorWallet = new TestAnchorWallet(keypair);

    // Get connected to the network and program
    const connection = new Connection("http://localhost:8899", "confirmed");

    // Create a DID identifier
    const didIdentifier = DidSolIdentifier.parse(testData.did);

    // Create a DID service
    const service = await DidSolService.build(didIdentifier, {
      connection,
      wallet: anchorWallet,
    });

    await service
      .addVerificationMethod({
        fragment: "key-2",
        keyData: keypair.publicKey.toBuffer(),
        methodType: VerificationMethodType.Ed25519VerificationKey2018,
        flags: [BitwiseVerificationMethodFlag.Assertion],
      })
      .withAutomaticAlloc(keypair.publicKey)
      .rpc();

    const did_doc = await service.resolve();

    console.log("did_doc:", did_doc);

    console.log("addVerificationMethod completed successfully");
  } catch (error) {
    console.error("Error during addVerificationMethod:", error);
    process.exit(1);
  }
}

addVerificationMethod();

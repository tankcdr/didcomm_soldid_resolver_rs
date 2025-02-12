import { Connection, Keypair } from "@solana/web3.js";
import { DidSolIdentifier, DidSolService } from "@identity.com/sol-did-client";
import fs from "fs";
import { TestAnchorWallet } from "./TestAnchorWallet";

async function addService() {
  try {
    // Read the test data
    const testData = JSON.parse(fs.readFileSync("test_data.json", "utf-8"));
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

    service.addService({
      fragment: "agent",
      serviceType: "TestService",
      serviceEndpoint: "https://test-service.com",
    });

    console.log("addService completed successfully");
  } catch (error) {
    console.error("Error during addService:", error);
    process.exit(1);
  }
}

addService();

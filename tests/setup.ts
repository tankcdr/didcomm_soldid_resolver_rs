import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  SendTransactionError,
} from "@solana/web3.js";
import {
  DidSolIdentifier,
  DidSolService,
  INITIAL_MIN_ACCOUNT_SIZE,
} from "@identity.com/sol-did-client";
import { TestAnchorWallet } from "./TestAnchorWallet";
import fs from "fs";

function loadKeypair(filePath: string): Keypair {
  const secretKeyString = fs.readFileSync(filePath, "utf8");
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return Keypair.fromSecretKey(secretKey);
}

async function setup() {
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

    // Initialize the DID account, explicitly passing the wallet for signing.
    await service.initialize(INITIAL_MIN_ACCOUNT_SIZE).rpc();

    let did_doc = await service.resolve();
    console.log(JSON.stringify(did_doc, null, 2));

    if (!did_doc) {
      throw new Error("Failed to create DID account");
    }

    console.log("Successfully created DID:", did_doc.id.toString());
  } catch (error) {
    console.error("Error creating DID:", error);

    if (error instanceof SendTransactionError) {
      console.error(
        "Transaction error:",
        await (error as SendTransactionError).getLogs(connection)
      );
    }
    process.exit(1);
  }
}

setup();

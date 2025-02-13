import { Keypair } from "@solana/web3.js";
import * as fs from "fs";

/**
 * Loads a Solana keypair from a JSON file.
 * @param filePath Path to the keypair JSON file.
 * @returns Keypair object
 */
export function loadKeypair(filePath: string): Keypair {
  const secretKeyString = fs.readFileSync(filePath, "utf8");
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return Keypair.fromSecretKey(secretKey);
}

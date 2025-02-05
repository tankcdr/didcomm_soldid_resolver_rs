// Constants
// Solana RPC URLs for different networks
pub const MAINNET_RPC: &str = "https://api.mainnet-beta.solana.com";
pub const TESTNET_RPC: &str = "https://api.testnet.solana.com";
pub const DEVNET_RPC: &str = "https://api.devnet.solana.com";
pub const LOCALNET_RPC: &str = "http://127.0.0.1:8899"; // Local validator
/// Regex to validate did:sol format
pub const DID_SOL_REGEX: &str =
    r"^did:sol(?::(testnet|devnet|localnet))?:([1-9A-HJ-NP-Za-km-z]{40,48})$";
// solana did program id
pub const DID_PROGRAM_ID: &str = "didso1Dpqpm4CsiCjzP766BGY89CAdD6ZBL68cRhFPc";

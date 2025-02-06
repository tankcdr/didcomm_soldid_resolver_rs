use crate::constants::*;
use crate::errors::*;
use crate::did_doc_builder::*;

use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;
use anchor_client::{ anchor_lang::AccountDeserialize, solana_client::rpc_client::RpcClient };
use didcomm::{ did::{ DIDDoc, DIDResolver }, error::Error, error::ErrorKind };
use regex::Regex;
use sol_did::state::DidAccount;
use async_trait::async_trait;
use log::{ info, debug };

// Resolver struct
pub struct SolResolver {}

impl Default for SolResolver {
    fn default() -> Self {
        Self {}
    }
}

impl SolResolver {
    // Function to derive the DID account address
    fn derive_did_account(did_pubkey: &Pubkey) -> (Pubkey, u8) {
        let seed = "did-account";
        Pubkey::find_program_address(
            &[seed.as_bytes(), did_pubkey.as_ref()],
            &Pubkey::from_str(DID_PROGRAM_ID).unwrap()
        )
    }
}

#[async_trait(?Send)]
impl DIDResolver for SolResolver {
    async fn resolve(
        &self,
        did: &str
    ) -> std::result::Result<Option<DIDDoc>, didcomm::error::Error> {
        debug!("Resolving DID: {}", did);

        // Validate DID format using regex
        let re = Regex::new(DID_SOL_REGEX).map_err(|_|
            Error::msg(ErrorKind::Malformed, SolResolverError::InvalidDidFormat.to_string())
        )?;

        let captures = re
            .captures(did)
            .ok_or_else(||
                Error::msg(ErrorKind::Malformed, SolResolverError::InvalidDidFormat.to_string())
            )?;

        // Extract optional network and address
        let network = captures
            .get(1)
            .map(|m| m.as_str())
            .unwrap_or("mainnet");

        let address = captures
            .get(2)
            .ok_or_else(||
                Error::msg(ErrorKind::Malformed, SolResolverError::InvalidSolanaAddress.to_string())
            )?
            .as_str();

        // Check if the address is valid before converting
        if address.len() < 32 || address.len() > 44 {
            return Err(
                Error::msg(ErrorKind::Malformed, "Invalid Solana address length".to_string())
            );
        }

        info!("Extracted Network: {}", network);
        info!("Extracted Address: {}", address);

        // Determine the appropriate RPC URL based on network
        let rpc_url = match network {
            "testnet" => TESTNET_RPC,
            "devnet" => DEVNET_RPC,
            "localnet" => LOCALNET_RPC,
            _ => MAINNET_RPC, // Default to mainnet if no network is specified
        };

        let rpc_client = RpcClient::new(rpc_url.to_string());

        // Convert extracted address to a Solana Pubkey
        let did_pubkey = Pubkey::from_str(address).map_err(|_m|
            Error::msg(ErrorKind::Malformed, SolResolverError::InvalidSolanaAddress.to_string())
        )?;
        info!("Derived Solana Pubkey: {:?}", did_pubkey);

        // Derive the DID account
        let (did_account_pubkey, _) = Self::derive_did_account(&did_pubkey);
        info!("Derived DID Account Pubkey: {:?}", did_account_pubkey);

        // After deriving did_account_pubkey
        let account_data_result = rpc_client.get_account_data(&did_account_pubkey);
        debug!("Account Data Result: {:?}", account_data_result);

        let did_document = match account_data_result {
            Ok(account_data) => {
                let did_account = DidAccount::try_deserialize(&mut account_data.as_ref()).map_err(
                    |m|
                        Error::msg(
                            ErrorKind::Malformed,
                            format!("Anchor Deserialization Error: {}", m)
                        )
                )?;

                DidDocBuilder::new(did, address).with_onchain_data(&did_account, did).build()
            }
            Err(_) => { DidDocBuilder::new(did, address).build() }
        };

        Ok(Some(did_document))
    }
}

use std::process::Command;
use std::fs;
use didcomm::did::DIDDoc;
use didcomm::did::DIDResolver;
use solana_sdk::signature::{ Keypair, Signer };
use didcomm_soldid_resolver::{ SolResolver, config };
use std::sync::Once;

// Used to ensure one-time initialization
static INIT: Once = Once::new();

pub(crate) struct TestContext {
    pub publicKey: String,
    pub did: String,
    pub resolver: SolResolver,
}

impl TestContext {
    pub async fn new(
        network: TestNetwork,
        data_file_name: &str
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize program ID only once
        INIT.call_once(|| {
            config::initialize_program_id(&network.program_id());
        });

        // Read the JSON file as a string
        let keypair_bytes: Vec<u8> = serde_json::from_str(&fs::read_to_string(data_file_name)?)?;
        // Convert the bytes to a Keypair
        let keypair = Keypair::from_bytes(&keypair_bytes)?;
        // Extract the public key and encode it in Base58
        let public_key_base58 = keypair.pubkey().to_string();

        let did = "did:sol:".to_string() + network.to_string() + ":" + &public_key_base58;

        Ok(Self {
            publicKey: public_key_base58,
            did,
            resolver: SolResolver::default(),
        })
    }

    pub fn run_npm_command(script: &str) -> Result<(), Box<dyn std::error::Error>> {
        let status = Command::new("npm").arg("run").arg(script).status()?;

        if !status.success() {
            return Err(format!("Failed to run {}", script).into());
        }
        Ok(())
    }

    pub fn setup(&self) -> Result<(), Box<dyn std::error::Error>> {
        TestContext::run_npm_command("test:setup")
    }

    pub fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        TestContext::run_npm_command("test:cleanup")
    }

    pub async fn add_verification_method(&self) -> Result<(), Box<dyn std::error::Error>> {
        TestContext::run_npm_command("test:add-verification-method")
    }

    pub async fn add_service(&self) -> Result<(), Box<dyn std::error::Error>> {
        TestContext::run_npm_command("test:add-service")
    }

    pub async fn resolve(&self) -> Result<Option<DIDDoc>, Box<dyn std::error::Error>> {
        let resolver = SolResolver::default();
        Ok(resolver.resolve(&self.did).await?)
    }
}

/// Enum representing allowed Solana networks
#[derive(Debug)]
pub enum TestNetwork {
    Localnet,
    Devnet,
}

impl TestNetwork {
    /// Parse network from a string
    fn from_str(network: &str) -> Result<Self, &'static str> {
        match network {
            "localnet" => Ok(TestNetwork::Localnet),
            "devnet" => Ok(TestNetwork::Devnet),
            _ => Err("Invalid network. Must be 'localnet' or 'devnet'"),
        }
    }

    /// Parse network from a string
    fn to_string(&self) -> &'static str {
        match self {
            TestNetwork::Localnet => "localnet",
            TestNetwork::Devnet => "devnet",
        }
    }

    /// Get the corresponding program ID based on the network
    fn program_id(&self) -> &'static str {
        match self {
            TestNetwork::Localnet => "LOCALNET_PROGRAM_ID_XXXXXXX",
            TestNetwork::Devnet => "didso1Dpqpm4CsiCjzP766BGY89CAdD6ZBL68cRhFPc",
        }
    }
}

pub(crate) struct TestGuard<'a> {
    pub context: &'a TestContext,
}

impl<'a> Drop for TestGuard<'a> {
    fn drop(&mut self) {
        if let Err(e) = self.context.cleanup() {
            eprintln!("Failed to clean up test context: {:?}", e);
        }
    }
}

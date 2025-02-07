# didcomm_soldid_resolver

A Rust library for resolving [Solana DID Documents](https://github.com/identity-com/sol-did) for DIDCOMM messaging. Implements the DIDResolver trait of the SICPA Digital Lab's implementtion of DIDComm. Additionally, a W3C Doc resolver is provided for use of the Solana DID outside of DIDCOMM scenarios.

## Quick Start

Using the DIDCOMM resolver:

```rust
use didcomm_soldid_resolver::SolResolver;
use didcomm::DIDResolver;

let resolver = SolResolver::default();
let did_doc = resolver.resolve("did:sol:BYJ3xJ9spKsmHqS7d3VejkPhLizqn9ZzE3QjaQp7iTuS").await?;
```

Using the W3C DID Document resolver:

```rust
use didcomm_soldid_resolver::W3cDidDocument;

let did_doc = W3cDidDocument::resolve("did:sol:BYJ3xJ9spKsmHqS7d3VejkPhLizqn9ZzE3QjaQp7iTuS").await?;
```

## Why Use This?

- **Cross-Chain Messaging**: Enables Solana DIDs to participate in cross-chain DIDCOMM messaging
- **Universal DID Resolution**: Works with any DIDCOMM application by implementing the standard DIDResolver trait
- **Network Flexibility**: Automatic network detection for mainnet, testnet, devnet, and local validator networks
- **Recovery-Friendly**: Gracefully handles both on-chain and chainless DID documents
- **Standards Compliant**: Fully compatible with [Solana DID Method Specification](https://github.com/identity-com/sol-did)

## Installation

Add to your Cargo.toml:

```toml
[dependencies]
didcomm_soldid_resolver = { git = "https://github.com/tankcdr/didcomm_soldid_resolver_rs.git" }
```

## Usage Examples

### Basic Resolution

```rust
let resolver = SolResolver::default();

// Mainnet resolution
let did_doc = resolver.resolve("did:sol:BYJ3xJ9spKsmHqS7d3VejkPhLizqn9ZzE3QjaQp7iTuS").await?;

// Devnet resolution
let did_doc = resolver.resolve("did:sol:devnet:BYJ3xJ9spKsmHqS7d3VejkPhLizqn9ZzE3QjaQp7iTuS").await?;
```

### Integration with DIDCOMM

```rust
use didcomm_soldid_resolver::SolResolver;
use didcomm::{Message, DIDResolver};

async fn send_message(to_did: &str, message: &str) -> Result<(), Error> {
    let resolver = SolResolver::default();
    let message = Message::new()
        .from("did:sol:sender")
        .to(to_did)
        .body(message)
        .build()?;

    // Resolver automatically handles DID resolution for message encryption
    let encrypted = message.encrypt(None, to_did, &resolver).await?;
    // Send encrypted message...
}
```

## Features

### Verification Method Support

- Ed25519VerificationKey2018
- EcdsaSecp256k1RecoveryMethod2020
- EcdsaSecp256k1VerificationKey2019

### Network Resolution

The resolver automatically detects and connects to the appropriate network:

```
did:sol:<address>           // Mainnet (default)
did:sol:devnet:<address>    // Devnet
did:sol:testnet:<address>   // Testnet
did:sol:localnet:<address>  // Local Validator
```

### Service Support

Full support for Solana DID service endpoints, with automatic mapping to DIDCOMM services.

## Development Status

This library is under active development. While it's being used in production, please report any issues via GitHub issues.

# Contributing

Contributions are welcome! We follow the standard GitHub pull request process:

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to your fork
5. Submit a pull request

Please ensure your PR:

- Includes relevant tests
- Updates documentation as needed
- Follows existing code style
- Has a clear description of changes

For major changes, please open an issue first to discuss the proposed changes.

## License

MIT License - see the [LICENSE](LICENSE) file for details.

## Resources

- [Solana DID Specification](https://g.identity.com/sol-did/)
- [DIDCOMM Messaging Specification](https://identity.foundation/didcomm-messaging/spec/)
- [DIDCOMM Rust Implementation](https://github.com/sicpa-dlab/didcomm-rust)

## ❤️ Support This Project

Maintaining and improving this project takes time and effort. If you find it useful and would like to support its development, consider sending a donation.

### 💸 Donate via Solana (SOL)

Send SOL donations to the following address: EuMmg13a2Vi7LPh64mwkRGsoPtYEUnAkyCwa7oYmsx58

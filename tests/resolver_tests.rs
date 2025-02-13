use didcomm::did::{ DIDResolver, VerificationMaterial };
use didcomm_soldid_resolver::SolResolver;

/* 
//not ready for this test yet
#[tokio::test]
async fn test_resolve_mainnet_did() {
    let resolver = SolResolver::default();
    let did = "did:sol:BYJ3xJ9spKsmHqS7d3VejkPhLizqn9ZzE3QjaQp7iTuS";

    let result = resolver.resolve(did).await;
    assert!(result.is_ok());

    if let Ok(Some(doc)) = result {
        assert_eq!(doc.id, did);
        assert!(!doc.authentication.is_empty());
        assert_eq!(doc.authentication[0], format!("{}#default", did));
    }
}*/

#[tokio::test(flavor = "multi_thread")]
async fn test_resolve_devnet_did() {
    let _ = env_logger::builder().is_test(true).try_init();

    let resolver = SolResolver::default();
    let did = "did:sol:devnet:2CE5VrAVc51cGCwk8JScajgpR8RuKmV1vxLPUpM8Lkxv";

    let result = resolver.resolve(did).await;
    assert!(result.is_ok());

    if let Ok(Some(doc)) = result {
        assert_eq!(doc.id, did);
        assert_eq!(doc.verification_method.len(), 1);
        assert!(doc.service.is_empty());
        assert_eq!(doc.verification_method[0].id, format!("{}#default", did));
        assert!(
            matches!(
                doc.verification_method[0].type_,
                didcomm::did::VerificationMethodType::Ed25519VerificationKey2018
            )
        );
        assert_eq!(doc.verification_method[0].controller, did);
        assert!(
            matches!(
                    &doc.verification_method[0].verification_material,
                    VerificationMaterial::Base58 { public_key_base58 } if public_key_base58 == "2CE5VrAVc51cGCwk8JScajgpR8RuKmV1vxLPUpM8Lkxv"
                )
        );
    }
}

/*
//not ready for these yet
#[tokio::test]
async fn test_resolve_testnet_did() {
    let resolver = SolResolver::default();
    let did = "did:sol:testnet:BYJ3xJ9spKsmHqS7d3VejkPhLizqn9ZzE3QjaQp7iTuS";

    let result = resolver.resolve(did).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_resolve_localnet_did() {
    let resolver = SolResolver::default();
    let did = "did:sol:localnet:BYJ3xJ9spKsmHqS7d3VejkPhLizqn9ZzE3QjaQp7iTuS";

    let result = resolver.resolve(did).await;
    assert!(result.is_ok());
}
*/

#[tokio::test(flavor = "multi_thread")]
async fn test_resolve_invalid_did_format() {
    let resolver = SolResolver::default();
    let did = "invalid:did:format";

    let result = resolver.resolve(did).await;
    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_resolve_invalid_address_length() {
    let resolver = SolResolver::default();
    let did = "did:sol:abc123"; // Too short address

    let result = resolver.resolve(did).await;
    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_resolve_non_base58_address() {
    let resolver = SolResolver::default();
    let did = "did:sol:!!!invalid$$$base58!!!address!!!";

    let result = resolver.resolve(did).await;
    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_chainless_did_resolution() {
    let resolver = SolResolver::default();
    // Using a random valid base58 address that likely doesn't have on-chain data
    let did = "did:sol:2CE5VrAVc51cGCwk8JScajgpR8RuKmV1vxLPUpM8Lkxv";

    let result = resolver.resolve(did).await;
    assert!(result.is_ok());

    if let Ok(Some(doc)) = result {
        assert_eq!(doc.id, did);
        assert_eq!(doc.verification_method.len(), 1);
        assert!(doc.service.is_empty());
    }
}

/// All dids have at least one verification method
#[tokio::test(flavor = "multi_thread")]
async fn test_did_with_default_verification_methods() {
    let _ = env_logger::builder().is_test(true).try_init();

    let resolver = SolResolver::default();
    // Use a known DID that has multiple verification methods
    let did = "did:sol:9VwGmqEarF7U8QE5RqZrrtYuGtumqxqQZ2G9Vm4d3Npi";

    let result = resolver.resolve(did).await;
    assert!(result.is_ok());

    if let Ok(Some(did_doc)) = result {
        assert_eq!(did_doc.id, did);
        assert_eq!(did_doc.verification_method.len(), 1);
        assert_eq!(did_doc.verification_method[0].id, format!("{}#default", did));
        assert!(
            matches!(
                did_doc.verification_method[0].type_,
                didcomm::did::VerificationMethodType::Ed25519VerificationKey2018
            )
        );
        assert_eq!(did_doc.verification_method[0].controller, did);
        assert!(
            matches!(
                    &did_doc.verification_method[0].verification_material,
                    VerificationMaterial::Base58 { public_key_base58 } if public_key_base58 == "9VwGmqEarF7U8QE5RqZrrtYuGtumqxqQZ2G9Vm4d3Npi"
                )
        );
    }
}

/* 
//TODO: need to create a DID with many verification methods to test this
#[tokio::test(flavor = "multi_thread")]
async fn test_did_with_multiple_verification_methods() {
    let resolver = SolResolver::default();
    // Use a known DID that has multiple verification methods
    let did = "did:sol:9VwGmqEarF7U8QE5RqZrrtYuGtumqxqQZ2G9Vm4d3Npi";

    let result = resolver.resolve(did).await;
    assert!(result.is_ok());
}
    */

/* 
//TODO: need to create a DID with services to test this
#[tokio::test(flavor = "multi_thread")]
async fn test_did_with_services() {
    let resolver = SolResolver::default();
    // Use a known DID that has service endpoints
    let did = "did:sol:7pGiVVyiZrXGXLruRqVCEzsyVR2hQaFemgUu5gVYnDxs";

    let result = resolver.resolve(did).await;
    assert!(result.is_ok());
}

    */

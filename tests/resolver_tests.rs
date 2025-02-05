use didcomm_soldid_resolver::SolResolver;
use didcomm::did::DIDResolver;

#[tokio::test(flavor = "multi_thread")]
async fn test_resolve_on_chain_did() {
    let _ = env_logger::builder().is_test(true).try_init();

    // Example DID (use a known Solana DID for testing)
    let did = "did:sol:devnet:7pGiVVyiZrXGXLruRqVCEzsyVR2hQaFemgUu5gVYnDxs";

    // Create a new SolResolver
    let resolver = SolResolver::default();

    let result = resolver.resolve(did).await;

    match result {
        Ok(Some(did_doc)) => {
            assert_eq!(did_doc.id, did);
            println!("✅ Test Passed! DID Document: {:?}", did_doc);
            let json_output = serde_json::to_string_pretty(&did_doc).unwrap();
            println!("{}", json_output);
        }
        Ok(None) => {
            panic!("❌ Test Failed! DID Document not found!");
        }
        Err(e) => {
            panic!("❌ Test Failed! Error resolving DID: {:?}", e);
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_resolve_chainless_did() {
    let _ = env_logger::builder().is_test(true).try_init();

    // Example DID (use a known Solana DID for testing)
    let did = "did:sol:devnet:7pGiVVyiZrXGXLruRqVCEzsyVR2hQaFemgUu5gVYnDxt";

    // Create a new SolResolver
    let resolver = SolResolver::default();

    let result = resolver.resolve(did).await;

    match result {
        Ok(Some(did_doc)) => {
            assert_eq!(did_doc.id, did);
            println!("✅ Test Passed! DID Document: {:?}", did_doc);
            let json_output = serde_json::to_string_pretty(&did_doc).unwrap();
            println!("{}", json_output);
        }
        Ok(None) => {
            panic!("❌ Test Failed! DID Document not found!");
        }
        Err(e) => {
            panic!("❌ Test Failed! Error resolving DID: {:?}", e);
        }
    }
}

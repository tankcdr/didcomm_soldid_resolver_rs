mod test_context;

use tokio;
use test_context::{ TestContext, TestGuard, TestNetwork };
use didcomm::did::{ VerificationMaterial, VerificationMethodType };

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_basic_did_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let context = TestContext::new(TestNetwork::Devnet, "./tests/data/test-account.json").await?;

    // Ensure cleanup always runs, even if a test fails
    let _guard = TestGuard { context: &context };

    context.setup()?;

    let result = context.resolve().await?;
    assert!(result.is_some());
    if let Some(doc) = result {
        assert_eq!(doc.id, context.did);

        // Verify that the document has a default verification method
        assert_eq!(doc.verification_method.len(), 1); // Should have default verification method
        let default_vm = doc.verification_method.get(0).unwrap();
        assert_eq!(default_vm.id, format!("{}#{}", context.did, "default"));
        assert_eq!(default_vm.controller, context.did);
        assert!(matches!(default_vm.type_, VerificationMethodType::Ed25519VerificationKey2018));
        // Verify that `verification_material` is a Base58 type and extract the value
        match &default_vm.verification_material {
            VerificationMaterial::Base58 { public_key_base58 } => {
                assert_eq!(*public_key_base58, context.publicKey);
            }
            _ => panic!("Verification material is not a Base58 key!"),
        }

        // Verify that the document has no services
        assert!(doc.service.is_empty());

        // Verify that the document has no authentication
        assert_eq!(doc.authentication.len(), 1);
        assert_eq!(doc.authentication[0], format!("{}#{}", context.did, "default"));

        // Verify that the document has no key agreement
        assert_eq!(doc.key_agreement.len(), 1);
        assert_eq!(doc.key_agreement[0], format!("{}#{}", context.did, "default"));
    }

    Ok(())
}

/* 
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_did_with_additional_verification_method() -> Result<(), Box<dyn std::error::Error>> {
    let context = TestContext::new(TestNetwork::Devnet, "./tests/data/test-account.json").await?;

    context.setup()?;

    // Add a verification method
    context.add_verification_method().await?;

    // Resolve and verify
    let result = context.resolve().await?;
    assert!(result.is_some());
    if let Some(doc) = result {
        print!("{:?}", doc);
        assert_eq!(doc.verification_method.len(), 2); // Default + new method
    }

    context.cleanup()?;
    Ok(())
}

 
#[tokio::test]
async fn test_full_did_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let context = TestContext::new().await?;

    // Test initial state
    let initial_doc = context.resolve().await?.unwrap();
    assert_eq!(initial_doc.verification_method.len(), 1);

    // Add verification method
    context.add_verification_method().await?;
    let updated_doc = context.resolve().await?.unwrap();
    assert_eq!(updated_doc.verification_method.len(), 2);

    // Add service
    context.add_service().await?;
    let final_doc = context.resolve().await?.unwrap();
    assert!(!final_doc.service.is_empty());

    context.cleanup()?;
    Ok(())
}
    */

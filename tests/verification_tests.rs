mod test_context;

use serde_json::Value;
use serial_test::serial;
use tokio;
use test_context::{ TestContext, TestGuard, TestNetwork };
use didcomm::did::{ ServiceKind, VerificationMaterial, VerificationMethodType };

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[serial]
async fn test_basic_did_resolution() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::Builder
        ::new()
        .filter_module("didcomm_soldid_resolver", log::LevelFilter::Debug) // Enable debug logs
        .is_test(true)
        .try_init();

    let context = TestContext::new(TestNetwork::Devnet, "./tests/data/test-account.json").await?;
    context.setup()?;

    // Ensure cleanup always runs, even if a test fails
    let _guard = TestGuard { context: &context };

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
        assert_eq!(doc.authentication.len(), 0);

        // Verify that the document has no key agreement
        assert_eq!(doc.key_agreement.len(), 0);
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[serial]
async fn test_did_with_additional_verification_method() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::Builder
        ::new()
        .filter_module("didcomm_soldid_resolver", log::LevelFilter::Debug) // Enable debug logs
        .is_test(true)
        .try_init();

    let context = TestContext::new(TestNetwork::Devnet, "./tests/data/test-account.json").await?;
    context.setup()?;

    // Ensure cleanup always runs, even if a test fails
    let _guard = TestGuard { context: &context };

    // Add a verification method
    context.add_verification_method().await?;

    // Resolve and verify
    let result = context.resolve().await?;

    assert!(result.is_some());
    if let Some(doc) = result {
        assert_eq!(doc.id, context.did);
        assert_eq!(doc.verification_method.len(), 2); // Default + new method

        if
            let Some(key2) = doc.verification_method
                .iter()
                .find(|vm| vm.id == format!("{}#{}", context.did, "key-2"))
        {
            assert_eq!(key2.controller, context.did);
            assert!(matches!(key2.type_, VerificationMethodType::Ed25519VerificationKey2018));
            match &key2.verification_material {
                VerificationMaterial::Base58 { public_key_base58 } => {
                    assert_eq!(*public_key_base58, context.publicKey);
                }
                _ => panic!("Verification material is not a Base58 key!"),
            }
        } else {
            panic!("Key 2 not found");
        }

        //script sets the KeyAgreement bit, thus assertionMethod should have 1 element
        assert_eq!(doc.key_agreement.len(), 1);
        assert_eq!(doc.key_agreement[0], format!("{}#{}", context.did, "key-2"));
        //note that the add verification script also sets the Assertion bit
        // but the Didcomm DidDoc does not have an assertion field
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[serial]
async fn test_did_with_additional_service() -> Result<(), Box<dyn std::error::Error>> {
    //setup debug logging
    let _ = env_logger::Builder
        ::new()
        .filter_module("didcomm_soldid_resolver", log::LevelFilter::Debug) // Enable debug logs
        .is_test(true)
        .try_init();

    //create testing context
    let context = TestContext::new(TestNetwork::Devnet, "./tests/data/test-account.json").await?;
    context.setup()?;

    // Ensure cleanup always runs, even if a test fails
    let _guard = TestGuard { context: &context };

    // Add service
    context.add_service().await?;

    // Resolve and verify
    let result = context.resolve().await?;

    assert!(result.is_some());
    if let Some(doc) = result {
        assert_eq!(doc.id, context.did);
        assert_eq!(doc.service.len(), 1); // Should have 1 service
        assert_eq!(doc.service[0].id, format!("{}#{}", context.did, "agent"));

        let extracted = extract_service_details(&doc.service[0].service_endpoint);
        assert!(extracted.is_some());

        let (id, service_endpoint, service_type) = extracted.unwrap();

        assert_eq!(id, "did:sol:devnet:2CE5VrAVc51cGCwk8JScajgpR8RuKmV1vxLPUpM8Lkxv#agent");
        assert_eq!(service_endpoint, "https://test-service.com");
        assert_eq!(service_type, "TestService");
    }

    Ok(())
}

fn extract_service_details(service_endpoint: &ServiceKind) -> Option<(String, String, String)> {
    if let ServiceKind::Other { value } = service_endpoint {
        if let Value::Object(map) = value {
            let id = map.get("id")?.as_str()?.to_string();
            let endpoint = map.get("serviceEndpoint")?.as_str()?.to_string();
            let service_type = map.get("type")?.as_str()?.to_string();
            return Some((id, endpoint, service_type));
        }
    }
    None
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[serial]
async fn test_full_did_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    //setup debug logging
    let _ = env_logger::Builder
        ::new()
        .filter_module("didcomm_soldid_resolver", log::LevelFilter::Debug) // Enable debug logs
        .is_test(true)
        .try_init();

    //create testing context
    let context = TestContext::new(TestNetwork::Devnet, "./tests/data/test-account.json").await?;
    context.setup()?;

    // Ensure cleanup always runs, even if a test fails
    let _guard = TestGuard { context: &context };
    // Add service
    context.add_service().await?;
    // Add verification method
    context.add_verification_method().await?;

    // Resolve and verify
    let result = context.resolve().await?;

    assert!(result.is_some());
    if let Some(doc) = result {
        assert_eq!(doc.id, context.did);
        assert_eq!(doc.service.len(), 1); // Should have 1 service
        assert_eq!(doc.service[0].id, format!("{}#{}", context.did, "agent"));

        assert_eq!(doc.id, context.did);
        assert_eq!(doc.verification_method.len(), 2); // Default + new method
    }

    Ok(())
}

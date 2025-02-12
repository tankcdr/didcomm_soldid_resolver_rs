use didcomm_soldid_resolver::{ W3cDidDocument, w3c_doc::PublicKeyFormat };

#[tokio::test(flavor = "multi_thread")]
async fn test_w3c_resolve_basic_did() {
    let did = "did:sol:devnet:BYJ3xJ9spKsmHqS7d3VejkPhLizqn9ZzE3QjaQp7iTuS";
    let result = W3cDidDocument::resolve(did).await;

    assert!(result.is_ok());
    if let Ok(doc) = result {
        assert_eq!(doc.id, did);
        assert!(doc.context.contains(&"https://www.w3.org/ns/did/v1".to_string()));
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_w3c_doc_context() {
    let did = "did:sol:BYJ3xJ9spKsmHqS7d3VejkPhLizqn9ZzE3QjaQp7iTuS";
    let result = W3cDidDocument::resolve(did).await;

    assert!(result.is_ok());
    if let Ok(doc) = result {
        assert!(doc.context.len() >= 2);
        assert!(doc.context.contains(&"https://www.w3.org/ns/did/v1".to_string()));
        assert!(
            doc.context.contains(&"https://w3id.org/security/suites/ed25519-2018/v1".to_string())
        );
    }
}

/* 
// not ready for this test yet
#[tokio::test]
async fn test_w3c_verification_methods() {
    let did = "did:sol:BYJ3xJ9spKsmHqS7d3VejkPhLizqn9ZzE3QjaQp7iTuS";
    let result = W3cDidDocument::resolve(did).await;

    assert!(result.is_ok());
    if let Ok(doc) = result {
        assert!(!doc.verification_method.is_empty());

        // Check first verification method
        let first_method = &doc.verification_method[0];
        assert!(first_method.id.starts_with(did));
        assert!(!first_method.controller.is_empty());
    }
}
*/

/* 
// not ready for this test yet
#[tokio::test]
async fn test_w3c_services() {
    // Use a DID known to have services
    let did = "did:sol:7pGiVVyiZrXGXLruRqVCEzsyVR2hQaFemgUu5gVYnDxs";
    let result = W3cDidDocument::resolve(did).await;

    assert!(result.is_ok());
    if let Ok(doc) = result {
        // Verify services exist and have required fields
        for service in doc.service {
            assert!(!service.id.is_empty());
            assert!(!service.type_.is_empty());
            assert!(!service.service_endpoint.is_empty());
        }
    }
}
    */

#[tokio::test(flavor = "multi_thread")]
async fn test_w3c_authentication() {
    let did = "did:sol:BYJ3xJ9spKsmHqS7d3VejkPhLizqn9ZzE3QjaQp7iTuS";
    let result = W3cDidDocument::resolve(did).await;

    assert!(result.is_ok());
    if let Ok(doc) = result {
        assert!(!doc.authentication.is_empty());
        // Default authentication should reference the DID
        assert!(doc.authentication[0].starts_with(did));
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_w3c_key_agreement() {
    let did = "did:sol:BYJ3xJ9spKsmHqS7d3VejkPhLizqn9ZzE3QjaQp7iTuS";
    let result = W3cDidDocument::resolve(did).await;

    assert!(result.is_ok());
    if let Ok(doc) = result {
        // Check if key agreement references are properly formatted
        for key in doc.key_agreement {
            assert!(key.starts_with(did));
        }
    }
}

#[tokio::test]
async fn test_w3c_resolve_invalid_did() {
    let did = "invalid:did:format";
    let result = W3cDidDocument::resolve(did).await;
    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_w3c_chainless_did() {
    let did = "did:sol:5xqXJR7TQZwZEyqWPJVWp6vRfv4tXiWbPwmypYPnGoVc";
    let result = W3cDidDocument::resolve(did).await;

    assert!(result.is_ok());
    if let Ok(doc) = result {
        assert_eq!(doc.id, did);
        assert_eq!(doc.authentication.len(), 1);
        assert_eq!(doc.authentication[0], format!("{}#default", did));
        assert_eq!(doc.verification_method.len(), 1);
        assert!(doc.service.is_empty());
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_w3c_default_verification_method_types() {
    // Test a DID known to have multiple verification method types
    let did = "did:sol:9VwGmqEarF7U8QE5RqZrrtYuGtumqxqQZ2G9Vm4d3Npi";
    let result = W3cDidDocument::resolve(did).await;
    assert!(result.is_ok());

    if let Ok(doc) = result {
        assert_eq!(doc.id, did);
        assert_eq!(doc.verification_method.len(), 1);
        assert_eq!(doc.verification_method[0].id, format!("{}#default", did));
        assert_eq!(doc.verification_method[0].type_, "Ed25519VerificationKey2018".to_string());

        assert_eq!(doc.verification_method[0].controller, did);
        assert!(
            matches!(
                &doc.verification_method[0].public_key,
                PublicKeyFormat::Base58 { public_key_base58 } if public_key_base58 == "9VwGmqEarF7U8QE5RqZrrtYuGtumqxqQZ2G9Vm4d3Npi"
            )
        );
    }
}

use didcomm::did::DIDResolver;
use didcomm::did::ServiceKind;
use serde::{ Serialize, Deserialize };
use crate::SolResolver;

#[derive(Debug, Serialize, Deserialize)]
pub struct W3cDidDocument {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub also_known_as: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub controller: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub verification_method: Vec<VerificationMethod>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authentication: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assertion_method: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub key_agreement: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub capability_invocation: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub capability_delegation: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service: Vec<Service>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublicKeyFormat {
    Base58 {
        #[serde(rename = "publicKeyBase58")]
        public_key_base58: String,
    },
    Multibase {
        #[serde(rename = "publicKeyMultibase")]
        public_key_multibase: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub controller: String,
    #[serde(flatten)]
    pub public_key: PublicKeyFormat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub service_endpoint: String,
}

impl W3cDidDocument {
    pub async fn resolve(did: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let resolver = SolResolver::default();
        let did_doc = resolver.resolve(did).await?.ok_or("DID document not found")?;

        // Convert to W3C format
        let mut doc = Self {
            context: Self::default_context(),
            id: did.to_string(),
            also_known_as: vec![],
            controller: vec![],
            verification_method: vec![],
            authentication: did_doc.authentication.clone(),
            assertion_method: vec![],
            key_agreement: did_doc.key_agreement,
            capability_invocation: did_doc.authentication.clone(),
            capability_delegation: vec![],
            service: vec![],
        };

        // Convert verification methods
        doc.verification_method = did_doc.verification_method
            .into_iter()
            .map(|vm| VerificationMethod {
                id: vm.id,
                type_: Self::verification_type_to_string(&vm.type_),
                controller: vm.controller,
                public_key: match vm.verification_material {
                    didcomm::did::VerificationMaterial::Base58 { public_key_base58 } =>
                        PublicKeyFormat::Base58 { public_key_base58 },
                    didcomm::did::VerificationMaterial::Multibase { public_key_multibase } =>
                        PublicKeyFormat::Multibase { public_key_multibase },
                    _ => PublicKeyFormat::Base58 { public_key_base58: String::new() },
                },
            })
            .collect();

        doc.service = did_doc.service
            .into_iter()
            .map(|svc| {
                match svc.service_endpoint {
                    ServiceKind::Other { value } =>
                        Service {
                            id: svc.id,
                            type_: value
                                .get("type")
                                .and_then(|t| t.as_str())
                                .unwrap_or("UnknownType")
                                .to_string(),
                            service_endpoint: value
                                .get("serviceEndpoint")
                                .and_then(|e| e.as_str())
                                .unwrap_or("")
                                .to_string(),
                        },
                    _ =>
                        Service {
                            id: svc.id,
                            type_: "UnknownType".to_string(),
                            service_endpoint: "".to_string(),
                        },
                }
            })
            .collect();

        Ok(doc)
    }

    fn verification_type_to_string(vm_type: &didcomm::did::VerificationMethodType) -> String {
        (
            match vm_type {
                didcomm::did::VerificationMethodType::JsonWebKey2020 => "JsonWebKey2020",
                didcomm::did::VerificationMethodType::X25519KeyAgreementKey2019 =>
                    "X25519KeyAgreementKey2019",
                didcomm::did::VerificationMethodType::Ed25519VerificationKey2018 =>
                    "Ed25519VerificationKey2018",
                didcomm::did::VerificationMethodType::EcdsaSecp256k1VerificationKey2019 =>
                    "EcdsaSecp256k1VerificationKey2019",
                didcomm::did::VerificationMethodType::X25519KeyAgreementKey2020 =>
                    "X25519KeyAgreementKey2020",
                didcomm::did::VerificationMethodType::Ed25519VerificationKey2020 =>
                    "Ed25519VerificationKey2020",
                didcomm::did::VerificationMethodType::Other => "Other",
            }
        ).to_string()
    }

    fn default_context() -> Vec<String> {
        vec![
            "https://www.w3.org/ns/did/v1".to_string(),
            "https://w3id.org/security/suites/ed25519-2018/v1".to_string()
        ]
    }
}

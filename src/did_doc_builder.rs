use didcomm::did::{ DIDDoc, VerificationMethod, VerificationMethodType };
use didcomm::did::ServiceKind;
use didcomm::did::VerificationMaterial;
use serde_json::json;
use sol_did::state::DidAccount;

pub struct DidDocBuilder {
    did_doc: DIDDoc,
}

impl DidDocBuilder {
    /// Creates a minimal DIDDoc with only the DID and a default verification method.
    pub fn new(did: &str, address: &str) -> Self {
        Self {
            did_doc: DIDDoc {
                id: did.to_string(),
                key_agreement: vec![format!("{}#default", did)],
                authentication: vec![format!("{}#default", did)],
                verification_method: vec![VerificationMethod {
                    id: format!("{}#default", did),
                    type_: VerificationMethodType::Ed25519VerificationKey2018,
                    controller: did.to_string(),
                    verification_material: VerificationMaterial::Base58 {
                        public_key_base58: address.to_string(),
                    },
                }],
                service: vec![],
            },
        }
    }

    /// Adds on-chain data without overwriting defaults
    pub fn with_onchain_data(mut self, did_account: &DidAccount, did: &str) -> Self {
        // Append on-chain verification methods (without overwriting defaults)
        self.did_doc.key_agreement.extend(
            did_account.verification_methods
                .iter()
                .filter(|vm| matches!(vm.flags & 0x02, 0x02))
                .map(|vm| format!("{}#{}", did, vm.fragment))
        );

        self.did_doc.authentication.extend(
            did_account.verification_methods
                .iter()
                .filter(|vm| matches!(vm.flags & 0x01, 0x01))
                .map(|vm| format!("{}#{}", did, vm.fragment))
        );

        self.did_doc.verification_method.extend(
            did_account.verification_methods
                .iter()
                .map(|vm| Self::to_didcomm_verification_method(did, vm))
        );

        self.did_doc.service.extend(
            did_account.services.iter().map(|service| Self::to_didcomm_service(did, service))
        );

        self
    }

    /// Returns the final DIDDoc.
    pub fn build(self) -> DIDDoc {
        self.did_doc
    }

    // Function to convert Solana DID verification method type to DIDComm verification method type
    fn to_didcomm_verification_method_type(
        soldid_vm_type: u8
    ) -> didcomm::did::VerificationMethodType {
        match soldid_vm_type {
            0 => didcomm::did::VerificationMethodType::Ed25519VerificationKey2018,
            1 => didcomm::did::VerificationMethodType::Other, // EcdsaSecp256k1RecoveryMethod2020
            2 => didcomm::did::VerificationMethodType::EcdsaSecp256k1VerificationKey2019,
            _ => didcomm::did::VerificationMethodType::Other,
        }
    }

    fn to_verification_material(soldid_vm_type: u8, key_data: &[u8]) -> VerificationMaterial {
        match soldid_vm_type {
            0 =>
                VerificationMaterial::Base58 {
                    public_key_base58: bs58::encode(key_data).into_string(),
                }, // Ed25519VerificationKey2018
            _ =>
                VerificationMaterial::Multibase {
                    public_key_multibase: format!("z{}", bs58::encode(key_data).into_string()),
                },
        }
    }

    /// Converts a Solana verification method to DIDComm verification method
    fn to_didcomm_verification_method(
        did: &str,
        vm: &sol_did::state::VerificationMethod
    ) -> didcomm::did::VerificationMethod {
        didcomm::did::VerificationMethod {
            id: format!("{}#{}", did, vm.fragment),
            type_: Self::to_didcomm_verification_method_type(vm.method_type),
            controller: did.to_string(),
            verification_material: Self::to_verification_material(vm.method_type, &vm.key_data),
        }
    }

    /// Converts a Solana service to DIDComm service
    fn to_didcomm_service(did: &str, service: &sol_did::state::Service) -> didcomm::did::Service {
        didcomm::did::Service {
            id: format!("{}#{}", did, service.fragment),
            service_endpoint: ServiceKind::Other {
                value: json!({
                    "id": format!("{}#{}", did, service.fragment),
                    "type": service.service_type,
                    "serviceEndpoint": service.service_endpoint.clone(),
                }),
            },
        }
    }
}

use crate::data_types::event_protocol_messages::{
    event_protocol_messages::{
        Contract
    },
    application_constructs::application_contracts::utility_types::WitnessUsers,
    signatures::{
        organization_cert::{
            OrganizationCertificate
        },
        sig::Sig
    }
};

use serde::{Deserialize, Serialize};

// contains the data being signed
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InteractionPreSig {
    pub contract: Contract,
    pub signer_channel_pubkey: String,
    pub witnesses: WitnessUsers,
    pub wit_node_sigs: ArrayOfWnSignituresBytes,
    pub org_cert: OrganizationCertificate,
    pub timeout: u32,
}

// contains the data and a signature, as well the the key to verify with
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InteractionSig {
    pub contract: Contract,
    pub signer_channel_pubkey: String,
    pub witnesses: WitnessUsers,
    pub wit_node_sigs: ArrayOfWnSignituresBytes,
    pub org_cert: OrganizationCertificate,
    pub timeout: u32,
    pub signer_did_pubkey: String,
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfWnSignituresBytes(pub Vec<Vec<u8>>);


impl Sig for InteractionSig {
    fn get_did_pubkey(&self) -> String {
        return self.signer_did_pubkey.clone();
    }
    fn get_channel_pubkey(&self) -> String {
        return self.signer_channel_pubkey.clone();
    }
}
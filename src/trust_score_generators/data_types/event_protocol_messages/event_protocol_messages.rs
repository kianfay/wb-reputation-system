use crate::trust_score_generators::data_types::event_protocol_messages::{
    application_messages::exchange_app_messages::{CompensationMsg},
    contracts::{
        utility_types::{PublicKey, UserOrWitnesses},
        exchange_app_contract::ExchangeContract,
        meeting_app_contract::MeetingContract
    },
    signatures::{
        interaction_sig::InteractionSig,
        witness_sig::WitnessSig
    }
};

use serde::{Deserialize, Serialize};

////
//// GENERIC MESSAGES
////

// The top level types, such as InteractionMsg and WitnessStatement, are common
// for all applications of the event protocol. The ApplicationMsg allows for
// the inclusion of application specific messages.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message{
    WitnessStatement {
        outcome: Outcome
    },
    InteractionMsg {
        contract: Contract,
        witnesses: WitnessUsers,
        wit_node_sigs: ArrayOfWnSignitures,
        tx_client_sigs: ArrayOfTxSignitures,
    },
    ApplicationMsg(ApplicationMsg)
}

////
//// APPLICATION UTLITY TYPES
////

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApplicationMsg {
    CompensationMsg(CompensationMsg)
}

////
//// MESSAGES UTLITY TYPES
////

// an array of bytes representing the pubkey of the participant
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WitnessUsers       (pub Vec<PublicKey>);
// signitures are also simply arrays of bytes
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfTxSignitures(pub Vec<InteractionSig>);
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArrayOfWnSignitures(pub Vec<WitnessSig>);
pub type Outcome = Vec<bool>;

////
//// GENERIC CONTRACT
////

// Each Contract kind is for a specific application. Storing the
// contracts as en emum allows for abstraction away from the 
// event protocol application. 
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Contract {
	ExchangeContract(ExchangeContract),
	MeetingContract(MeetingContract)
}

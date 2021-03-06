use crate::data_types::event_protocol_messages::
	application_constructs::application_contracts::{
		utility_types::{
			ParticipantUsers, CompensationJson, UnixTimestamp,
			CoordinateDMSFormat
		}
};

use serde::{Deserialize, Serialize};

////
//// EXCHANGE APPLICATION CONTRACT
////
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExchangeContract {
	// main fields
	pub channel_address: String,
	pub offer: String,
	pub participants: ParticipantUsers,
	pub compensation: CompensationJson,
	// metadata fields
	pub time: UnixTimestamp,
	pub location: CoordinateDMSFormat,
	pub timeout: UnixTimestamp
}
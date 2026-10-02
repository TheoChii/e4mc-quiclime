use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
pub enum ServerboundControlMessage {
    ProbeCapabilities,
    RequestDomainAssignment {
        #[serde(default)]
        requested_domain: Option<String>,
        #[serde(default)]
        token: Option<String>,
    },
    DialtoneRegisterTicket { ticket: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
pub enum ClientboundControlMessage {
    UnknownMessage,
    HasCapabilities { caps: Vec<String> },
    DomainAssignmentComplete { domain: String },
    DomainAssignmentRejected { reason: String },
    RequestMessageBroadcast { message: String },
    TicketRegistered,
}

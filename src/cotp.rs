    
#[derive(Clone, Debug)]
pub enum Tpdu {
    ConnectionRequest,
    ConnectionConfirm,
    DisconnectRequest,
    DisconnectConfirm,
    Data,
    ExpeditedDate,
    Acknowledge,
    ExpeditedAcknowledge,
    Reject,
    Error(String)
}

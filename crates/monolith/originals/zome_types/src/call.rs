use monolith::holochain_zome_types::capability::CapSecret;
use monolith::holochain_zome_types::cell::CellId;
use monolith::holochain_zome_types::zome::FunctionName;
use monolith::holochain_zome_types::zome::ZomeName;
use holo_hash::AgentPubKey;
use holochain_serialized_bytes::prelude::SerializedBytes;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Call {
    pub to_cell: Option<CellId>,
    pub zome_name: ZomeName,
    pub fn_name: FunctionName,
    pub cap: Option<CapSecret>,
    pub request: SerializedBytes,
    pub provenance: AgentPubKey,
}

impl Call {
    pub fn new(
        to_cell: Option<CellId>,
        zome_name: ZomeName,
        fn_name: FunctionName,
        cap: Option<CapSecret>,
        request: SerializedBytes,
        provenance: AgentPubKey,
    ) -> Self {
        Self {
            to_cell,
            zome_name,
            fn_name,
            cap,
            request,
            provenance,
        }
    }
}

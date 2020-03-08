use crate::conductor::Conductor;
use async_trait::async_trait;
use mockall::mock;
use std::sync::Arc;
use sx_cell::cell::Cell;
use sx_conductor_api::{CellConductorInterfaceT, CellT, ConductorApiResult, ConductorT};
use sx_types::{
    agent::CellId,
    autonomic::AutonomicCue,
    nucleus::{ZomeInvocation, ZomeInvocationResponse},
    shims::*,
    signature::Signature,
};

mod api_cell;
mod api_external;
pub use api_cell::*;
pub use api_external::*;


// Unfortunate workaround to get mockall to work with async_trait, due to the complexity of each.
// The mock! expansion here creates mocks on a non-async version of the API, and then the actual trait is implemented
// by delegating each async trait method to its sync counterpart
// See https://github.com/asomers/mockall/issues/75
mock! {

    CellConductorInterface {

        fn sync_conductor(&self) -> tokio::sync::RwLockReadGuard<'static, Conductor>;

        fn sync_invoke_zome(
            &self,
            cell_id: &CellId,
            invocation: ZomeInvocation,
        ) -> ConductorApiResult<ZomeInvocationResponse>;

        fn sync_network_send(&self, message: Lib3hClientProtocol) -> ConductorApiResult<()>;

        fn sync_network_request(
            &self,
            _message: Lib3hClientProtocol,
        ) -> ConductorApiResult<Lib3hServerProtocol>;

        fn sync_autonomic_cue(&self, cue: AutonomicCue) -> ConductorApiResult<()>;

        fn sync_crypto_sign(&self, _payload: String) -> ConductorApiResult<Signature>;

        fn sync_crypto_encrypt(&self, _payload: String) -> ConductorApiResult<String>;

        fn sync_crypto_decrypt(&self, _payload: String) -> ConductorApiResult<String>;
    }

    trait Clone {
        fn clone(&self) -> Self;
    }
}

#[async_trait(?Send)]
impl CellConductorInterfaceT for MockCellConductorInterface {
    type Cell = Cell;
    type Conductor = Conductor;


    async fn conductor(&self) -> tokio::sync::RwLockReadGuard<'static, Conductor> {
        self.sync_conductor()
    }


    async fn invoke_zome(
        &self,
        cell_id: &CellId,
        invocation: ZomeInvocation,
    ) -> ConductorApiResult<ZomeInvocationResponse> {
        self.sync_invoke_zome(cell_id, invocation)
    }

    async fn network_send(&self, message: Lib3hClientProtocol) -> ConductorApiResult<()> {
        self.sync_network_send(message)
    }

    async fn network_request(
        &self,
        _message: Lib3hClientProtocol,
    ) -> ConductorApiResult<Lib3hServerProtocol> {
        self.sync_network_request(_message)
    }

    async fn autonomic_cue(&self, cue: AutonomicCue) -> ConductorApiResult<()> {
        self.sync_autonomic_cue(cue)
    }

    async fn crypto_sign(&self, _payload: String) -> ConductorApiResult<Signature> {
        self.sync_crypto_sign(_payload)
    }

    async fn crypto_encrypt(&self, _payload: String) -> ConductorApiResult<String> {
        self.sync_crypto_encrypt(_payload)
    }

    async fn crypto_decrypt(&self, _payload: String) -> ConductorApiResult<String> {
        self.sync_crypto_decrypt(_payload)
    }
}

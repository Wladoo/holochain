use sx_conductor_api::CellT;
use crate::conductor::Conductor;
use async_trait::async_trait;
use futures::sink::SinkExt;
use sx_cell::cell::{Cell, CellId};

use parking_lot::RwLock;
use std::sync::Arc;
use sx_conductor_api::{
    CellConductorInterfaceT, ConductorApiError, ConductorApiResult, ConductorT,
};
use sx_types::{
    autonomic::AutonomicCue,
    nucleus::{ZomeInvocation, ZomeInvocationResponse},
    shims::*,
    signature::Signature,
};

#[derive(Clone)]
pub struct CellConductorInterface {
    lock: Arc<RwLock<Conductor>>,
    cell_id: CellId,
}

impl CellConductorInterface {
    pub fn new(lock: Arc<RwLock<Conductor>>, cell_id: CellId) -> Self {
        Self { cell_id, lock }
    }
}

#[async_trait(?Send)]
impl CellConductorInterfaceT for CellConductorInterface {
    async fn invoke_zome(
        &self,
        cell_id: CellId,
        invocation: ZomeInvocation,
    ) -> ConductorApiResult<ZomeInvocationResponse> {
        let conductor = self.lock.read();
        let cell = conductor.cell_by_id(&cell_id)?;
        Ok(cell.invoke_zome(self.clone(), invocation).await?)
    }

    async fn network_send(&self, message: Lib3hClientProtocol) -> ConductorApiResult<()> {
        let mut tx = self.lock.read().tx_network().clone();
        tx.send(message)
            .await
            .map_err(|e| ConductorApiError::Misc(e.to_string()))
    }

    async fn network_request(
        &self,
        _message: Lib3hClientProtocol,
    ) -> ConductorApiResult<Lib3hServerProtocol> {
        unimplemented!()
    }

    async fn autonomic_cue(&self, cue: AutonomicCue) -> ConductorApiResult<()> {
        let conductor = self.lock.write();
        let cell = conductor.cell_by_id(&self.cell_id)?;
        let _ = cell.handle_autonomic_process(cue.into()).await;
        Ok(())
    }

    async fn crypto_sign(&self, _payload: String) -> ConductorApiResult<Signature> {
        unimplemented!()
    }

    async fn crypto_encrypt(&self, _payload: String) -> ConductorApiResult<String> {
        unimplemented!()
    }

    async fn crypto_decrypt(&self, _payload: String) -> ConductorApiResult<String> {
        unimplemented!()
    }
}


#[async_trait]
impl CellT for Cell {
    type Interface = CellConductorInterface;

    async fn invoke_zome(
        &self,
        _conductor_api: Self::Interface,
        _invocation: ZomeInvocation,
    ) -> ConductorApiResult<ZomeInvocationResponse> {
        unimplemented!()
    }
}

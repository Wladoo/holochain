use std::sync::Arc;

use crate::core::ribosome::{
    check_clone_access::check_clone_access, error::RibosomeError, CallContext, RibosomeT,
};
use holochain_types::{
    access::{HostFnAccess, Permission},
    app::EnableCloneCellPayload,
};
use holochain_util::tokio_helper;
use holochain_wasmer_host::prelude::wasm_error;
use holochain_wasmer_host::prelude::*;
use holochain_zome_types::clone::{ClonedCell, EnableCloneCellInput};
use wasmer::RuntimeError;

#[cfg_attr(feature = "instrument", tracing::instrument(skip(_ribosome, call_context), fields(? call_context.zome, function = ? call_context.function_name)))]
pub fn enable_clone_cell(
    _ribosome: Arc<impl RibosomeT>,
    call_context: Arc<CallContext>,
    input: EnableCloneCellInput,
) -> Result<ClonedCell, RuntimeError> {
    match HostFnAccess::from(&call_context.host_context()) {
        HostFnAccess {
            write_workspace: Permission::Allow,
            ..
        } => {
            let host_context = call_context.host_context();

            let conductor_handle = host_context.call_zome_handle();
            let (installed_app_id, _) =
                check_clone_access(conductor_handle.cell_id(), conductor_handle)?;

            tokio_helper::block_forever_on(async move {
                conductor_handle
                    .enable_clone_cell(
                        &installed_app_id,
                        EnableCloneCellPayload {
                            clone_cell_id: input.clone_cell_id,
                        },
                    )
                    .await
            })
            .map_err(|conductor_error| -> RuntimeError {
                wasm_error!(WasmErrorInner::Host(conductor_error.to_string())).into()
            })
        }
        _ => Err(wasm_error!(WasmErrorInner::Host(
            RibosomeError::HostFnPermissions(
                call_context.zome.zome_name().clone(),
                call_context.function_name().clone(),
                "enable_clone_cell".into(),
            )
            .to_string(),
        ))
        .into()),
    }
}

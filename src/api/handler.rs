
use dev_prefix::*;
use jsonrpc_core::{IoHandler, RpcMethodSync, Params, Error as RpcError};
use serde_json;

use export::ArtifactData;

use super::ARTIFACTS;

/// the rpc initializer that implements the API spec
fn init_rpc_handler() -> IoHandler {
    // partof: #SPC-rpc-artifacts
    let mut handler = IoHandler::new();
    handler.add_method("GetArtifacts", GetArtifacts);
    // TODO: update is disabled until it is feature complete
    // (specifically security needs to be added)
    // handler.add_method("UpdateArtifacts", update::UpdateArtifacts);
    handler
}

lazy_static! {
    pub static ref RPC_HANDLER: IoHandler = init_rpc_handler();
}

/// `GetArtifacts` API Handler
struct GetArtifacts;
impl RpcMethodSync for GetArtifacts {
    fn call(&self, _: Params) -> result::Result<serde_json::Value, RpcError> {
        info!("GetArtifacts called");
        let locked = ARTIFACTS.lock().unwrap();
        let artifacts: &Vec<ArtifactData> = locked.as_ref();
        Ok(serde_json::to_value(artifacts).expect("serde"))
    }
}

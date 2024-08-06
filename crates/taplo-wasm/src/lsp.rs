use crate::environment::WasmEnvironment;
use futures::Sink;
use js_sys::Function;
use lsp_async_stub::{rpc, Server};
use std::{io, sync::Arc};
use taplo_lsp::world::WorldState;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
pub struct TaploWasmLsp {
    pub(crate) server: Server<Arc<WorldState<WasmEnvironment>>>,
    pub(crate) world: Arc<WorldState<WasmEnvironment>>,
    pub(crate) lsp_interface: WasmLspInterface,
}

#[wasm_bindgen]
impl TaploWasmLsp {
    pub fn send(&self, message: JsValue) -> Result<(), JsError> {
        let message: lsp_async_stub::rpc::Message = serde_wasm_bindgen::from_value(message)?;
        let world = self.world.clone();
        let writer = self.lsp_interface.clone();

        let msg_fut = self.server.handle_message(world, message, writer);

        spawn_local(async move {
            if let Err(err) = msg_fut.await {
                tracing::error!(error = %err, "lsp message error");
            }
        });

        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct WasmLspInterface {
    js_on_message: Function,
}

impl From<JsValue> for WasmLspInterface {
    fn from(val: JsValue) -> Self {
        Self {
            js_on_message: js_sys::Reflect::get(&val, &JsValue::from_str("js_on_message"))
                .unwrap()
                .into(),
        }
    }
}

impl Sink<rpc::Message> for WasmLspInterface {
    type Error = io::Error;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn start_send(
        self: std::pin::Pin<&mut Self>,
        message: rpc::Message,
    ) -> Result<(), Self::Error> {
        let this = JsValue::null();
        self.js_on_message
            .call1(&this, &serde_wasm_bindgen::to_value(&message).unwrap())
            .unwrap();
        Ok(())
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
}

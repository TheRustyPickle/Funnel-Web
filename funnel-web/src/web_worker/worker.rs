use gloo_worker::{HandlerId, Worker, WorkerScope};

use crate::web_worker::handler_worker_comms;
use crate::web_worker::WorkerMessage;

pub struct WebWorker {}

impl Worker for WebWorker {
    type Message = WorkerMessage;
    type Input = WorkerMessage;
    type Output = WorkerMessage;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        let scope = scope.to_owned();
        wasm_bindgen_futures::spawn_local(async move {
            handler_worker_comms(msg, scope.to_owned(), id).await;
        });
    }
}

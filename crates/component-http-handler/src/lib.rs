wit_bindgen::generate!({
    path: "../../wit",
    world: "http-client",
    exports: {
        "wasi:http/incoming-handler": HttpClient,
    }
});

use wasi::http::types::RequestOptions;

use crate::exports::wasi::http::incoming_handler::{Guest, IncomingRequest};

struct HttpClient;
impl Guest for HttpClient {
    fn handle(req: IncomingRequest, opts: Option<RequestOptions>) {}
}




use exports::yeseh::wasm_research::petstore_client::{ErrorCode, Pet, PetStatus };
use wasi::clocks::monotonic_clock::{subscribe_duration, subscribe_instant};
use wasi::http::outgoing_handler::{handle, OutgoingRequest};
use wasi::http::types::{Headers, Scheme, FutureIncomingResponse};
use serde_json;

wit_bindgen::generate!({
  path: "../../wit",
  world: "petstore",
  exports: {
    "yeseh:wasm-research/petstore-client": Petstore,
  }
});


struct Petstore;
impl exports::yeseh::wasm_research::petstore_client::Guest for Petstore {
    fn find_pets_by_status(status:PetStatus,) -> Result<wit_bindgen::rt::vec::Vec::<Pet>,ErrorCode> {
        let headers = Headers::new(); 
        headers.append(&String::from("Content-Type"), &b"application/json".to_vec());
        headers.append(&String::from("Host"), &b"application/json".to_vec());

        let path = String::from(format!("/v2/pet/findByStatus?status={}", status));

        let req = OutgoingRequest::new(headers);
        req.set_authority(Some("petstore.swagger.io"));
        req.set_path_with_query(Some(&path));
        req.set_scheme(Some(&Scheme::Https));

        // Call handle from outgoing-handler
        let response_fut = handle(req, None)?;
        // In lieu of wasi-async we use block() to wait for the response
        let res_poll = response_fut.subscribe().block();
        if let Some(data) = response_fut.get() {
            // We only call .get once so we can unwrap()
            // If we call it again, it will return Err 
            let response = data.unwrap()?;
            let body = response.consume();
            let pets = serde_json::from_slice::<Vec<Pet>>(&body).unwrap();

            return Ok(pets);
        } else {
            Err(ErrorCode::Unknown)
        }
    }
}

  
#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::http::incoming_handler;

struct Component;

impl incoming_handler::Guest for Component {
    fn handle(
        request: incoming_handler::IncomingRequest,
        response_out: incoming_handler::ResponseOutparam,
    ) {
        // do some auth
        println!("Do some auth here!");

        // Invoke wrapped API
        bindings::wasi::http::incoming_handler::handle(request, response_out);
    }
}

bindings::export!(Component with_types_in bindings);

use tiny_http::{Server, Response};
use uwuifier::uwuify_str_sse;

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();
    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );
    
        let response = Response::from_string(uwuify_str_sse("We cannot solve problems with the kind of thinking we employed when we came up with them - Albert Einstein"));
        request.respond(response);
    }
}

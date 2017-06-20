extern crate bytes;
extern crate httpbis;
extern crate tls_api_openssl;
extern crate futures;

use bytes::Bytes;
use httpbis::{Header, Headers, Client, HttpScheme};
use futures::future::Future;

fn main() {

    let mut headers = Headers(vec![
        Header::new(Bytes::from_static(b":method"), Bytes::from_static(b"POST")),
        Header::new(Bytes::from_static(b":path"), Bytes::from_static(b"/helloworld.Greeter/SayHello")),
        Header::new(Bytes::from_static(b":authority"), Bytes::from_static(b"localhost")),
        Header::new(Bytes::from_static(b":scheme"), Bytes::from_static(b"http")),
        Header::new(Bytes::from_static(b"content-type"), Bytes::from_static(b"application/grpc")),
    ]);

    println!("header {:?}", headers);

    let client = httpbis::Client::new_plain("localhost", 50051, Default::default()).expect("Client");

    let mut mem = Bytes::from(&b"Hello world"[..]);
    let response = client.start_request_simple(headers, mem);

    println!("Response {:?}", response.collect().wait().unwrap().dump());

    //TODO start_request_simple with correct GRPC request bytes!
    //let resp = client.start_get("/helloworld.Greeter/SayHello", "localhost").collect().wait().expect("execute request");
}

extern crate bytes;
extern crate httpbis;
extern crate tls_api_openssl;
extern crate futures;

use self::bytes::{Bytes, BytesMut, BigEndian};
use self::bytes::BufMut;
use self::httpbis::{Header, Headers};
use self::futures::future::Future;

// for testing
pub fn send_dummy_request() {
    let headers = Headers(vec![
        Header::new(Bytes::from_static(b":method"), Bytes::from_static(b"POST")),
        Header::new(Bytes::from_static(b":path"), Bytes::from_static(b"/helloworld.Greeter/SayHello")),
        Header::new(Bytes::from_static(b":authority"), Bytes::from_static(b"localhost")),
        Header::new(Bytes::from_static(b":scheme"), Bytes::from_static(b"http")),
        Header::new(Bytes::from_static(b"content-type"), Bytes::from_static(b"application/grpc")),
    ]);

    println!("header {:?}", headers);

    let client = httpbis::Client::new_plain("localhost", 50051, Default::default()).expect("Client");

    let test_string = "Rustsfsf";
    let mut message = BytesMut::with_capacity(test_string.len()+1);
    println!("tag {:?}", 1u8 << 3 | & 2);
    message.put_u8(1u8 << 3 | & 2);
    message.put(test_string.len() as u8); //TODO use num bytes of string!
    message.put(test_string);

    println!("message {:?}", message.to_vec());
    let mut mem = BytesMut::with_capacity(5 + message.len());
    mem.put_u8(0);
    mem.put_u32::<BigEndian>(message.len() as u32);
    mem.put(message);

    println!("mem {:?}", mem.to_vec());

    let response = client.start_request_simple(headers, mem.freeze());

    println!("Response {:?}", response.collect().wait().unwrap().dump());

    //let resp = client.start_get("/helloworld.Greeter/SayHello", "localhost").collect().wait().expect("execute request");
}

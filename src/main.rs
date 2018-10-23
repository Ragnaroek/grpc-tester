extern crate clap;

mod grpc;

use clap::{Arg, App};

fn main() {

    let matches = App::new("grpc-tester")
                    .version("0.0.1")
                    .about("Easy GRPC testing")
                    .arg(Arg::with_name("proto")
                          .required(true)
                          .short("p")
                          .long("proto")
                          .value_name("PROTO_FILE")
                          .help("proto file with service and message definition")
                          .takes_value(true))
                    .arg(Arg::with_name("serveraddr")
                          .required(true)
                          .short("s")
                          .long("serveraddr")
                          .value_name("SERVER_ADDR")
                          .help("http address to GRPC server (e.g. localhost:8080)")
                          .takes_value(true))
                    .arg(Arg::with_name("rpcmessage")
                          .required(true)
                          .short("m")
                          .long("rpcmessage")
                          .help("name of the rpc message (e.g. SayHello)")
                          .takes_value(true))
                    .get_matches();

    println!("matches {:?}", matches);
    //grpc::send_dummy_request();
}

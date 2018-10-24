extern crate clap;

mod grpc;

use clap::{Arg, App, SubCommand};

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
                    .subcommand(SubCommand::with_name("send")
                        .about("sending grpc requests")
                        .arg(Arg::with_name("MESSAGENAME")
                            .help("GRPC message name")
                            .required(true))
                        .arg(Arg::with_name("ADDRESS")
                            .help("server address")
                            .default_value("localhost:8080")
                            .short("a")
                            .long("address")
                            .value_name("ADDRESS")
                            .takes_value(true))
                )
                .get_matches();

    //TODO: Read proto file (always?)

    if let Some(sub) = matches.subcommand_matches("send") {
        println!("send subcommand invoked");
        //grpct —proto /file.proto send <rpcmessagename> [—addr localhost:8080] << data from stdin
        //TODO Read message data from stdin
        //TODO create grpc request and send
    } else {
        println!("please select a subcommand")
    }
}

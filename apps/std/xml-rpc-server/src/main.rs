extern crate xml_rpc;
#[macro_use]
extern crate serde_derive;

use std::{net, thread};
use xml_rpc::{Client, Fault, Server};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TestStruct {
    pub foo: i32,
    pub bar: String,
}

fn echo(v: TestStruct) -> Result<TestStruct, Fault> {
    Ok(v)
}

fn double(mut v: TestStruct) -> Result<TestStruct, Fault> {
    v.foo *= 2;
    v.bar = format!("{0}{0}", v.bar);
    Ok(v)
}

pub fn main() {
    let socket = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 5555);
    let mut server = Server::new();
    server.register_simple("echo", echo);
    server.register_simple("double", double);
    let handler = thread::spawn(move || {
        let bound_server = server.bind(&socket).unwrap();
        let socket = bound_server.local_addr();
        println!("{}", socket);
        bound_server.run()
    });

    handler.join().unwrap();
    
}
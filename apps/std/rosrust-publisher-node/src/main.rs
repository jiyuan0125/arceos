//! A "hello world" echo server with Tokio
//!
//! This server will create a TCP listener, accept connections in a loop, and
//! write back everything that's read off of each TCP connection.
//!
//! Because the Tokio runtime uses a thread pool, each TCP connection is
//! processed concurrently with all other TCP connections across multiple
//! threads.
//!
//! To see this server in action, you can run this in one terminal:
//!
//!     make A=apps/std/tokio-echoserver STD=y NET=y run
//!
//! and in another terminal you can run:
//!
//!     cargo run --example connect 127.0.0.1:5555
//!
//! Each line you type in to the `connect` terminal should be echo'd back to
//! you! If you open up multiple terminals running the `connect` example you
//! should be able to see them all make progress simultaneously.

#![warn(rust_2018_idioms)]

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;

use xmlrpc::{Request, Value};

// run with: make A=apps/std/rosrust-publisher-node STD=y NET=y ACCEL=n build

fn main() {
    // The Python example server exports Python's `pow` method. Let's call it!
    let pow_request = Request::new("pow").arg(2).arg(8); // Compute 2**8

    let request_result = pow_request.call_url("http://127.0.0.1:8000");

    println!("Result: {:?}", request_result);

    let pow_result = request_result.unwrap();
    assert_eq!(pow_result, Value::Int(2i32.pow(8)));
}
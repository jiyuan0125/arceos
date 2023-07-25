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
//!     make A=apps/std/rosrust-publisher-node STD=y NET=y ACCEL=n run
//!
//! and in another terminal you can run:
//!
//!     cargo run --example connect 127.0.0.1:5555
//!
//! Each line you type in to the `connect` terminal should be echo'd back to
//! you! If you open up multiple terminals running the `connect` example you
//! should be able to see them all make progress simultaneously.

fn main() {
    // 使用命令 make A=apps/std/rosrust-publisher-node STD=y NET=y ACCEL=n build 编译
    // 报错: 
    // LLVM ERROR: Do not know how to split the result of this operator!
    // error: could not compile `rosrust-publisher-node` (bin "rosrust-publisher-node")
    // make: *** [scripts/make/build.mk:43: _cargo_build] Error 101
    rosrust::init("talker");
}

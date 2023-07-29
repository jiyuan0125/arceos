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
//! Each line you type in to the `connect` terminal should be echo'd back to
//! you! If you open up multiple terminals running the `connect` example you
//! should be able to see them all make progress simultaneously.

const ROS_MASTER_URI: &str = "http://10.0.2.2:11311";

fn main() {

    println!("starting!");

    // Initialize node
    rosrust::init_with_master_uri("talker", ROS_MASTER_URI);
    println!("initialized!");

    // Create publisher
    let chatter_pub = rosrust::publish("chatter", 2).unwrap();
    chatter_pub.wait_for_subscribers(None).unwrap();

    let log_names = rosrust::param("~log_names").unwrap().get().unwrap_or(false);

    let mut count = 0;

    // Create object that maintains 10Hz between sleep requests
    let rate = rosrust::rate(10.0);

    // Breaks when a shutdown signal is sent
    while rosrust::is_ok() {
        // Create string message
        let msg = rosrust_msg::std_msgs::String {
            data: format!("hello world from rosrust {}", count),
        };

        // Log event
        rosrust::ros_info!("Publishing: {}", msg.data);

        // Send string message to topic via publisher
        chatter_pub.send(msg).unwrap();

        if log_names {
            rosrust::ros_info!("Subscriber names: {:?}", chatter_pub.subscriber_names());
        }

        // Sleep to maintain 10Hz rate
        rate.sleep();

        count += 1;
    }
}
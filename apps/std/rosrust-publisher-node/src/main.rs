// export CARGO_CFG_HTTPARSE_DISABLE_SIMD=1
// make A=apps/std/rosrust-publisher-node STD=y NET=y LOG=info SMP=2 ACCEL=n run
const ROS_MASTER_URI: &str = "http://10.0.2.2:11311";
const HOSTNAME: &str = "127.0.0.1";
const SLAVE_PORT: u16 = 5555;

fn main() {
    println!("starting!");

    // Initialize node
    rosrust::init_with_master_uri_and_hostname_and_slave_port(
        "talker",
        ROS_MASTER_URI,
        HOSTNAME,
        SLAVE_PORT,
    );
    println!("initialized!");

    // Create publisher
    let chatter_pub = rosrust::publish("chatter", 2).unwrap();
    chatter_pub.wait_for_subscribers(None).unwrap();

    let log_names = rosrust::param("~log_names").unwrap().get().unwrap_or(false);

    let mut count = 0;

    // Create object that maintains 10Hz between sleep requests
    let rate = rosrust::rate(1.0);

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

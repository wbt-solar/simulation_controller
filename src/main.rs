mod redis_interface;
use redis_interface::RedisInterface;

fn main() {
    let redis_interface = RedisInterface::new();
    let reids_interface_clone = redis_interface.clone();
    // Start a thread to subscribe to a channel
    std::thread::spawn(move || {
        reids_interface_clone.subscribe("my_channel");
    });

    loop{
        // Publish a message to the channel
        redis_interface.publish("my_channel", "Hello, world!");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
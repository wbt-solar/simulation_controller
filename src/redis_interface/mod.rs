use redis::{Commands};
use colored::*;

pub struct RedisInterface {
    client: redis::Client,
}

impl Clone for RedisInterface {
    fn clone(&self) -> Self {
        RedisInterface {
            client: self.client.clone(),
        }
    }
}

impl RedisInterface {
    pub fn new() -> Self {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        RedisInterface { client }
    }

    pub fn subscribe(&self, channel: &str) {
        let mut con = self.client.get_connection().unwrap();
        let mut pubsub = con.as_pubsub();
        pubsub.subscribe(channel).unwrap();

        loop {
            let msg = pubsub.get_message().unwrap();
            let payload: String = msg.get_payload().unwrap();
            println!("{}", format!("Received: {}", payload).green());
        }
    }

    pub fn publish(&self, channel: &str, message: &str) {
        let mut con = self.client.get_connection().unwrap();
        let _: () = con.publish(channel, message).unwrap();
        println!("{}", format!("Sent: {}", message).blue());
    }
}
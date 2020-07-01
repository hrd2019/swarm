use std::thread;
use std::time::Duration;
use crate::httpclient::httpclient::{Httpserver};

mod httpclient;

fn main() {
    println!("Hello, world!");

    let config = swarm::parse_config();
    println!("{:#?}", config);

    let hs = Httpserver::new(config.hosts[0].as_str().unwrap(), config.max_num as i64);
    hs.run();
}

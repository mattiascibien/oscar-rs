#[macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;

use dotenv::dotenv;

use std::{thread,time};

fn main() {
    env_logger::init();

    dotenv().ok();

    loop {
        info!("Running");
        try_retweet();
        thread::sleep(time::Duration::from_secs(5*60));
    }
}

fn try_retweet() {

}

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate gotham;
extern crate hyper;
extern crate mime;

use std::env;
use std::{thread,time};

use dotenv::dotenv;

use hyper::{Response, StatusCode};

use gotham::http::response::create_response;
use gotham::state::State;

fn main() {
    env_logger::init();

    dotenv().ok();

    thread::spawn(move || {
        loop {
            info!("Running");
            try_retweet();
            thread::sleep(time::Duration::from_secs(5*60));
        }
    });

    let addr =  format!("{}{}", "0.0.0.0:", env::var("PORT").ok().unwrap_or("8080".to_string()));
    gotham::start(addr, || Ok(say_hello))
}

fn try_retweet() {

}

fn say_hello(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Oscarbot running.").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

//! Oasis - Open Sesame Bug Bounty Platform

extern crate futures;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mime;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

mod middlewares;
mod handlers;
mod router;

use self::router::router;

fn main() {
    env_logger::init();

    let addr = "127.0.0.1:8888";
    info!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}


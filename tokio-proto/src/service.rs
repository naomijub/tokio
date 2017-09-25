extern crate tokio_service;
extern crate futures;

use self::tokio_service::Service;
use self::futures::{future, Future};
use std::io;

pub struct EchoService;

impl Service for EchoService {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error =  Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        Box::new(future::ok(req))
    }
}

#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate tokio_timer;

use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};
use tokio_timer::*;
use futures::*;
use std::time::*;

static INDEX: &'static [u8] = b"Try POST /echo";

#[derive(Clone, Copy)]
struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Response, Error = hyper::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Get, "/") | (&Get, "/echo") => {
                Box::new(futures::future::ok(Response::new()
                    .with_header(ContentLength(INDEX.len() as u64))
                    .with_body(INDEX)))
            }
            (&Post, "/echo") => {
                let timer = Timer::default();
                println!("Press Ctrl-C on client now...");
                Box::new(timer.sleep(Duration::from_millis(2000))
                    .and_then(|_| {
                        println!("Sleep done (should not happen if client Ctrl-C'd)");
                        futures::future::ok(Response::new().with_body(req.body()))
                    })
                    .or_else(|e| {
                        let body = format!("Sleep failed: {}", e);
                        println!("{}", body);
                        futures::future::ok(Response::new().with_body(body))
                    }))
            }
            _ => Box::new(futures::future::ok(Response::new().with_status(StatusCode::NotFound))),
        }
    }
}

fn main() {
    pretty_env_logger::init().unwrap();
    let addr = "127.0.0.1:1337".parse().unwrap();

    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    println!("Listening on http://{} with 1 thread.",
             server.local_addr().unwrap());
    server.run().unwrap();
}

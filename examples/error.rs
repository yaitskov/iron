#![feature(phase)]

#[phase(plugin, link)] extern crate log;
extern crate iron;
extern crate http;
extern crate time;

use std::io::net::ip::Ipv4Addr;
use iron::{Iron, Handler, BeforeMiddleware, IronError,
           Request, Response, ChainBuilder, Chain,
           IronResult, Error};
use iron::status;

struct ErrorHandler;
struct ErrorProducer;

impl Handler for ErrorHandler {
    fn call(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::new())
    }

    fn catch(&self, _: &mut Request, err: IronError) -> (Response, IronResult<()>) {
        (Response::with(status::InternalServerError, "Internal Server Error."), Err(err))
    }

}

impl BeforeMiddleware for ErrorProducer {
    fn before(&self, _: &mut Request) -> IronResult<()> {
        Err("Error".to_string().abstract())
    }
}

fn main() {
    // Handler is attached here.
    let mut chain = ChainBuilder::new(ErrorHandler);

    // Link our error maker.
    chain.link_before(ErrorProducer);

    Iron::new(chain).listen(Ipv4Addr(127, 0, 0, 1), 3000);
    println!("On 3000");
}


extern crate iron;
extern crate build;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
  	let details = build::build_as_json();
        Ok(Response::with((status::Ok, details)))
    }

    let _server = Iron::new(hello_world).http("localhost:3000").unwrap();
}


extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate rand;

use iron::prelude::*;
use iron::status;
use rand::distributions::{IndependentSample, Range};
use mount::Mount;
use staticfile::Static;
use std::path::Path;

const HEIGHT: i32 = 480;
const WIDTH: i32 = 512;
const BOUNDARY: i32 = 32;

fn main() {

    // Handle get requests by sending back a random position on the map in json format.
    fn handler(_request: &mut Request) -> IronResult<Response> {
        let y_range = Range::new(BOUNDARY, HEIGHT-2*BOUNDARY); // Range for y coordinate
        let x_range = Range::new(BOUNDARY, WIDTH-2*BOUNDARY); // Range for x coordinate
        let mut rng = rand::thread_rng(); // Random number generator

        // Get a random point
        let x = x_range.ind_sample(&mut rng);
        let y = y_range.ind_sample(&mut rng);
        // Put together response
        let response = format!("{{\"x\":\"{}\", \"y\":\"{}\"}}", x, y).clone();
        // Send it
        Ok(Response::with((status::Ok, response)))
    }

    let mut mount = Mount::new();
    // Serve the game at /game/
    mount.mount("/game/", Static::new(Path::new("simple_game")));
    // Serve game server at /
    mount.mount("/", handler);
    Iron::new(mount).http("127.0.0.1:4000").unwrap();
}

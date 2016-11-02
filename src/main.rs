
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate rand;
// extern crate time;

extern crate rustc_serialize;
use rustc_serialize::json;

use iron::prelude::*;
use iron::status;
use rand::distributions::{IndependentSample, Range};
use mount::Mount;
use staticfile::Static;
use std::path::Path;

use std::time::{Duration, Instant};
use std::thread::sleep;

const HEIGHT: u64 = 480;
const WIDTH: u64 = 512;
const BOUNDARY: u64 = 32;

struct Game {
    hero: Point,
    gnome: Point,
    speed: u64,
    time: Instant,
}

#[derive(RustcDecodable, RustcEncodable)]
struct View {
    hero: Point,
    gnome: Point,
}

#[derive(RustcDecodable, RustcEncodable, Copy, Clone)]
struct Point {
    x: u64,
    y: u64,
}

fn main() {

    let mut game = Game {
        hero: Point {x: WIDTH/2, y: HEIGHT/2},
        gnome: Point {x: 3, y: 4},
        speed: 256,
        time: Instant::now(),
    };
    let mut view = game.get_view();

    let encoded = json::encode(&view).unwrap();
    println!("{:?}", encoded);
    game.update();
    view = game.get_view();
    let encoded = json::encode(&view).unwrap();
    println!("Updated: {:?}", encoded);

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

    //let mut mount = Mount::new();
    // Serve the game at /game/
    //mount.mount("/game/", Static::new(Path::new("simple_game")));
    // Serve game server at /
    //mount.mount("/", handler);
    //Iron::new(mount).http("127.0.0.1:4000").unwrap();
}

impl Game {
    fn update(&mut self) {
        sleep(Duration::new(2, 0));
        let delta_t = self.time.elapsed().as_secs();
        let delta_x = self.speed * delta_t;
        self.hero.x = self.hero.x + delta_x;
        self.time = Instant::now();
    }

    fn get_view(&self) -> View {
        let view = View {
            hero: self.hero,
            gnome: self.gnome,
        };
        return view;
    }
}

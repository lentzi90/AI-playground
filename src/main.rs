extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate rand;
extern crate rustc_serialize;

use rustc_serialize::json;

use iron::prelude::*;
use iron::status;
use iron::{BeforeMiddleware, typemap};
use mount::Mount;
use staticfile::Static;
use std::path::Path;

use std::io::Write;
use std::net::TcpListener;
use std::thread;
use std::thread::sleep;

use rand::distributions::{IndependentSample, Range};
use std::time::{Duration, Instant};

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

    println!("Serving game");
    let mut mount = Mount::new();
    // Serve the game at /game/
    mount.mount("/game/", Static::new(Path::new("simple_game")));

    // Serve game server at /
    // mount.mount("/", handler);
    thread::spawn(move || {
        Iron::new(mount).http("127.0.0.1:9000").unwrap();
    });

    println!("Initializing game");
    let mut game: Game = Game {
        hero: Point {x: WIDTH/2, y: HEIGHT/2},
        gnome: Point {x: 3, y: 4},
        speed: 256,
        time: Instant::now(),
    };
    println!("Starting server...");
    // game.run_server();
    let mut chain = Chain::new(handler);
    chain.link_before(game);
    Iron::new(chain).http("127.0.0.1:4000").unwrap();
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
        // self.update();
        View {
            hero: self.hero,
            gnome: self.gnome,
        }
    }

    fn run_server(&mut self) {
        let address = "127.0.0.1:4000";
        let listener = TcpListener::bind(address).unwrap();
        println!("Listening on {}", address);
        for stream in listener.incoming() {
            thread::spawn(move || {
                let mut stream = stream.unwrap();
                let response = format!("This is a response\n");
                stream.write(response.as_bytes()).unwrap();
            });
        }
    }
}

impl typemap::Key for View { type Value = u64; }

impl BeforeMiddleware for Game {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let view: View = self.get_view();
        println!("Extracted view {:?}", view.hero.x);
        req.extensions.insert::<View>(314157);
        Ok(())
    }
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let data = *req.extensions.get::<View>().unwrap();
    let response = format!("This is a response with data: {}\n", data);
    Ok(Response::with((iron::status::Ok, response)))
}


// Handle get requests by sending back a random position on the map in json format.
// fn handler(_request: &mut Request) -> IronResult<Response> {
//     let y_range = Range::new(BOUNDARY, HEIGHT-2*BOUNDARY); // Range for y coordinate
//     let x_range = Range::new(BOUNDARY, WIDTH-2*BOUNDARY); // Range for x coordinate
//     let mut rng = rand::thread_rng(); // Random number generator
//
//     // Get a random point
//     let x = x_range.ind_sample(&mut rng);
//     let y = y_range.ind_sample(&mut rng);
// }
//
//  Json encoding
// unsafe {
//     let mut view = game.get_view();
//
//     let encoded = json::encode(&view).unwrap();
//     println!("{:?}", encoded);
//     game.update();
//     view = game.get_view();
//     let encoded = json::encode(&view).unwrap();
//     println!("Updated: {:?}", encoded);
// }

extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate rand;
extern crate rustc_serialize;

use rustc_serialize::json;

use iron::prelude::*;
use mount::Mount;
use staticfile::Static;
use std::path::Path;

use std::thread;
use std::thread::sleep;
use std::sync::{Arc, Mutex};

// use rand::distributions::{IndependentSample, Range};
use std::time::{Duration, Instant};

const HEIGHT: u64 = 480;
const WIDTH: u64 = 512;
// const BOUNDARY: u64 = 32;

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

    println!("Initializing game");
    let game: Game = Game {
        hero: Point {x: WIDTH/2, y: HEIGHT/2},
        gnome: Point {x: 3, y: 4},
        speed: 256,
        time: Instant::now(),
    };

    let game_mtx = Arc::new(Mutex::new(game));
    let game_clone = game_mtx.clone();

    thread::spawn(move || {
        let address = "127.0.0.1:4000";
        println!("Starting game server at {}", address);
        let mut mount = Mount::new();
        mount.mount("/", move |r: &mut Request| handle_get(r, &game_clone.lock().unwrap()));
        Iron::new(mount).http(address).unwrap();
    });

    thread::spawn(move || {
        let address = "127.0.0.1:9000";
        println!("Serving game view {}", address);
        let mut mount = Mount::new();
        // Serve the game at /game/
        mount.mount("/", Static::new(Path::new("simple_game")));
        Iron::new(mount).http(address).unwrap();
    });

    for _ in 0..10 {
        sleep(Duration::new(2, 0));
        println!("Updating game");
        game_mtx.lock().unwrap().update();
    }

}

impl Game {
    fn update(&mut self) {
        // sleep(Duration::new(2, 0));
        let delta_t = self.time.elapsed().as_secs();
        let delta_x = self.speed * delta_t;
        self.hero.x = self.hero.x + delta_x;
        self.time = Instant::now();
    }

    fn get_view(&self) -> View {
        View {
            hero: self.hero,
            gnome: self.gnome,
        }
    }
}

fn handle_get(_: &mut Request, game: &Game) -> IronResult<Response> {
    let view: View = game.get_view();
    let response = json::encode(&view).unwrap();
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

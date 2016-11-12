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
use std::io::Read;

// use rand::distributions::{IndependentSample, Range};
use std::time::{Duration, Instant};

const HEIGHT: u64 = 480;
const WIDTH: u64 = 512;
// const BOUNDARY: u64 = 32;

struct Game {
    hero: Point,
    gnome: Point,
    speed: f64,
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

#[derive(RustcDecodable)]
struct Speed {
    speed: f64
}

fn main() {

    println!("Initializing game");
    let game: Game = Game {
        hero: Point {x: WIDTH/2, y: HEIGHT/2},
        gnome: Point {x: 3, y: 4},
        speed: 0.005,
        time: Instant::now(),
    };

    let game_mtx = Arc::new(Mutex::new(game));
    let game_clone = game_mtx.clone();
    let game_clone2 = game_mtx.clone();

    thread::spawn(move || {
        let address = "127.0.0.1:4000";
        let folder = "/data/";
        println!("Serving game view {}", address);
        println!("Starting game server at {}{}", address, folder);
        let mut mount = Mount::new();
        mount.mount(folder, move |r: &mut Request| handle_get(r, &game_clone.lock().unwrap()));
        mount.mount("/set/", move |r: &mut Request| handle_set(r, &mut game_clone2.lock().unwrap()));
        mount.mount("/", Static::new(Path::new("simple_game")));
        Iron::new(mount).http(address).unwrap();
    });

    loop {
        sleep(Duration::new(0, 100_000_000));
        // println!("Updating game");
        game_mtx.lock().unwrap().update();
    }

}

impl Game {
    fn update(&mut self) {
        let secs = self.time.elapsed().as_secs();
        let millis = (self.time.elapsed().subsec_nanos()/1_000) as u64;
        // delta t in seconds
        let delta_t: f64 = (secs + millis/1_000) as f64;
        let delta_x = (self.speed * delta_t).round() as i64;
        self.hero.x = (self.hero.x as i64 + delta_x) as u64;
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

fn handle_set(request: &mut Request, game: &mut Game) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    println!("Got message: {:?}", payload);
    let speed: Speed = json::decode(&payload).unwrap();
    game.speed = speed.speed;
    Ok(Response::with((iron::status::Ok)))
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

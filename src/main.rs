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

use rand::distributions::{IndependentSample, Range};
use std::time::{Duration, Instant};

const HEIGHT: u32 = 480;
const WIDTH: u32 = 512;
const BOUNDARY: u32 = 32;

struct Game {
    hero: Point,
    gnome: Point,
    speed: Speed,
    score: u32,
    time: Instant,
}

#[derive(RustcDecodable, RustcEncodable)]
struct View {
    hero: Point,
    gnome: Point,
    score: u32,
}

#[derive(RustcDecodable, RustcEncodable, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(RustcDecodable, RustcEncodable, Copy, Clone)]
struct Speed {
    amplitude: f64,
    direction: f64 // in radians
}

fn main() {

    println!("Initializing game");
    let game: Game = Game {
        hero: Point {x: WIDTH/2, y: HEIGHT/2},
        gnome: Point {x: 300, y: 200},
        speed: Speed {amplitude: 0.0, direction: 0.0},
        score: 0,
        time: Instant::now(),
    };

    // Clone game and pass clones to iron
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

    // Run the game
    loop {
        sleep(Duration::new(0, 10_000_000));
        game_mtx.lock().unwrap().update();
    }

}

impl Game {

    /// Update the game
    fn update(&mut self) {
        self.update_hero();
        self.update_gnome();
    }

    /// Return a View of the game
    fn get_view(&self) -> View {
        View {
            hero: self.hero,
            gnome: self.gnome,
            score: self.score,
        }
    }

    /// Update the position of the hero on the map
    fn update_hero(&mut self) {

        let width = WIDTH as i64;
        let height = HEIGHT as i64;

        // How long since last update?
        let secs = self.time.elapsed().as_secs();
        let millis = (self.time.elapsed().subsec_nanos()/1_000) as u64;
        // delta t in seconds
        let delta_t: f64 = (secs + millis/1_000) as f64;
        // how long have the hero moved?
        let radius = self.speed.amplitude * delta_t;
        // in what direction?
        let theta = self.speed.direction;

        // calculate the x and y component
        let delta_x = (radius * theta.cos()).round() as i64;
        let delta_y = (radius * theta.sin()).round() as i64;

        // new coordinates
        let mut x = (self.hero.x as i64 + delta_x) as i64;
        let mut y = (self.hero.y as i64 + delta_y) as i64;

        // Flip the hero over to the other side if moving outside of the map
        if x > width {
            x = x % width;
        } else if x < 0 {
            x = x + width;
        }
        if y > height {
            y = y % height;
        } else if y < 0 {
            y = y + height;
        }

        assert!((x >= 0) & (y >= 0), "Negative coordinate!");

        self.hero.x = x as u32;
        self.hero.y = y as u32;
        self.time = Instant::now();
    }

    /// Add a new gnome if needed
    fn update_gnome(&mut self) {
        let hero = self.hero;
        let gnome = self.gnome;

        if (hero.x <= (gnome.x + 32)) & (gnome.x <= (hero.x + 32))
                & (hero.y <= (gnome.y + 32)) & (gnome.y <= (hero.y + 32)) {

            let y_range = Range::new(BOUNDARY, HEIGHT-2*BOUNDARY); // Range for y coordinate
            let x_range = Range::new(BOUNDARY, WIDTH-2*BOUNDARY); // Range for x coordinate
            let mut rng = rand::thread_rng(); // Random number generator

            // Get a random point
            let x = x_range.ind_sample(&mut rng);
            let y = y_range.ind_sample(&mut rng);
            self.gnome.x = x;
            self.gnome.y = y;
            // Add score
            self.score += 1;
    	}
    }
}

/// Handle get requests by sending back the current view
fn handle_get(_: &mut Request, game: &Game) -> IronResult<Response> {
    let view: View = game.get_view();
    let response = json::encode(&view).unwrap();
    Ok(Response::with((iron::status::Ok, response)))
}

/// Handle a request to update the speed of the hero
fn handle_set(request: &mut Request, game: &mut Game) -> IronResult<Response> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload).unwrap();
    let speed: Speed = json::decode(&payload).unwrap();
    game.speed = speed;
    Ok(Response::with((iron::status::Ok)))
}

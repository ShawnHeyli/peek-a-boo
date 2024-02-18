use std::{
    fs::{self, File},
    path::Path,
};

use rand::seq::SliceRandom;

mod camera;
mod circular_buffer;
mod parser;

fn main() {
    // Check if config.txt exists
    let config_file = "config.txt";
    if !Path::new(config_file).exists() {
        // Create config file and write default camera
        File::create(config_file).unwrap();
        fs::write(
            config_file,
            "🗻 http://66.119.104.154/axis-cgi/jpg/image.cgi 24",
        )
        .unwrap();
    }

    let cameras = parser::read_config_file(config_file).unwrap();
    // Choose a camera at random
    let camera = cameras.choose(&mut rand::thread_rng()).unwrap();
    loop {
        println!("Setting wallpaper with {}", camera);
        camera.set_as_wallpaper();
        let sleep_time = 1000 / camera.refresh_rate.unwrap_or(24);
        println!("Sleeping for {}ms", sleep_time);
        std::thread::sleep(std::time::Duration::from_millis(sleep_time as u64));
    }
}

use std::{
    fs::File,
    path::Path, time::Instant,
};
use rand::seq::SliceRandom;

mod camera;
mod circular_buffer;
mod parser;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Check if config.txt exists
    let config_file = "config.txt";
    if !Path::new(config_file).exists() {
        // Create config file and write default camera
        File::create(config_file).unwrap();
        println!("Created config file, please add cameras to it");
    }

    let cameras = parser::read_config_file(config_file).unwrap();
    // Choose a camera at random
    let camera = cameras.choose(&mut rand::thread_rng()).unwrap();
    loop {
        println!("Setting wallpaper with {}", camera);
        let start = Instant::now();
        camera.set_as_wallpaper().await;
        let end = start.elapsed().as_millis() as i128;
        let sleep_time = (1000 / camera.refresh_rate.unwrap_or(24) as i128) - end;
        println!("Sleeping for {}ms", sleep_time);
        if sleep_time > 0 {
            std::thread::sleep(std::time::Duration::from_millis(sleep_time.try_into().unwrap()));
        }
    }
}

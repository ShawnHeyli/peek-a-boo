use std::io::{self, BufRead};

use crate::camera::Camera;

// Name, URL, refresh rate(optional)(in frames per second)
pub fn read_config_file(filename: &str) -> io::Result<Vec<Camera>> {
    let file = std::fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut cameras = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap().to_string();
        let url = parts.next().unwrap().to_string();
        let refresh_rate = parts.next().map(|s| s.parse().unwrap());
        cameras.push(Camera::new(name, url, refresh_rate));
    }

    Ok(cameras)
}

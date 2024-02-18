use std::fmt::Display;
use reqwest::get;
use convert_case::{Case, Casing};

#[derive(Debug)]
pub struct Camera {
    name: String,
    url: String,
    pub refresh_rate: Option<usize>,
}

impl Camera {
    pub fn new(name: String, url: String, refresh_rate: Option<usize>) -> Self {
        Self {
            name: name.to_case(Case::Snake),
            url,
            refresh_rate,
        }
    }

    async fn get_image(&self) {
        let img_bytes = get(&self.url).await.unwrap().bytes().await.unwrap();
        let image = image::load_from_memory(&img_bytes).unwrap();
        image.resize_to_fill(1920, 1080, image::imageops::FilterType::Lanczos3);
        image.save(format!("{}.jpg", &self.name)).unwrap();
    }

    pub async fn set_as_wallpaper(&self) {
        self.get_image().await;
        let output = std::process::Command::new("swww")
            .arg("img")
            .args(["--transition-fps", "60"])
            .args(["--resize", "fit"])
            .arg(format!("{}.jpg", &self.name))
            .output()
            .expect("Failed to set wallpaper with swww");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

impl Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Camera {} at {}", self.name, self.url)
    }
}

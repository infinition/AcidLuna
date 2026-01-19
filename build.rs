use std::path::Path;
use image::{RgbaImage, ImageFormat};

fn main() {
    let icon_path = "icon.ico";
    
    // Générer l'icône si elle n'existe pas
    if !Path::new(icon_path).exists() {
        let width = 32;
        let height = 32;
        let mut img = RgbaImage::new(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - 16;
                let dy = y as i32 - 16;
                let distance_sq = dx * dx + dy * dy;
                if distance_sq < 14 * 14 {
                    img.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
                } else {
                    img.put_pixel(x, y, image::Rgba([0, 0, 0, 0]));
                }
            }
        }
        img.save_with_format(icon_path, ImageFormat::Ico).expect("Failed to save icon.ico");
    }

    // Configurer les ressources Windows
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon(icon_path);
        res.compile().unwrap();
    }
}

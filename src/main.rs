use std::env;
use image::{Rgb};
use imageproc::drawing::{draw_filled_circle_mut};

fn main() {
    let image_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("No image path provided. Using default image.");
            "imgs/fishes.jpg".to_owned()
        }
    };
    let mut img = image::open(&image_path)
        .expect("No image found at provided path")
        .to_rgb8();
    let red   = Rgb([255u8, 0u8, 0u8]);
    draw_filled_circle_mut(&mut img, (150, 150), 150, red);
    println!("Circle drawn!");
    img.save("imgs/visual_poetry.jpg");
    println!("Image saved");
}

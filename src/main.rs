use std::env;
use image::Rgb;
use imageproc::drawing::draw_filled_circle_mut;

mod graphics;
use crate::graphics::Graphics;

mod utils;
use crate::utils::Utils;

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
    let white = Rgb([255u8, 255u8, 255u8]);
    draw_filled_circle_mut(&mut img, (150, 150), 150, red);
    println!("Date is: {:}", Utils::get_date());
    println!("Circle drawn!");
    Graphics::draw_rects(&mut img, 60, 10, 250, 200, 40, white);
    println!("Rects drawn!");
    img.save("imgs/visual_poetry.jpg");
    println!("Image saved");
}
